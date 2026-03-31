use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

use glob::Pattern;

use guardrail3_app_rs_family_mapper::RsReleaseRoute;
use guardrail3_domain_project_tree::ProjectTree;

use super::types::{CargoRootFacts, ReleaseInputFailureFacts};
use crate::release_support::binaries::join_under_root;

pub(super) fn collect_cargo_roots(
    tree: &ProjectTree,
    route: &RsReleaseRoute,
    input_failures: &mut Vec<ReleaseInputFailureFacts>,
) -> BTreeMap<String, CargoRootFacts> {
    let mut attempted_rel_dirs = BTreeSet::new();
    let mut cargo_roots = BTreeMap::new();
    let mut pending_rel_dirs = cargo_manifest_owner_dirs(tree, route)
        .into_iter()
        .collect::<Vec<_>>();

    while let Some(candidate_rel_dir) = pending_rel_dirs.pop() {
        if !attempted_rel_dirs.insert(candidate_rel_dir.clone()) {
            continue;
        }
        if let Some(root) = parse_root(tree, &candidate_rel_dir, input_failures) {
            for member_rel in expanded_workspace_members(tree, &root) {
                if !attempted_rel_dirs.contains(&member_rel) {
                    pending_rel_dirs.push(member_rel);
                }
            }
            let _ = cargo_roots.insert(candidate_rel_dir, root);
        }
    }

    for claimed_rel_dir in claimed_workspace_packages(tree, cargo_roots.values()) {
        if attempted_rel_dirs.insert(claimed_rel_dir.clone()) {
            if let Some(root) = parse_root(tree, &claimed_rel_dir, input_failures) {
                let _ = cargo_roots.insert(claimed_rel_dir, root);
            }
        }
    }

    cargo_roots
}

fn cargo_manifest_owner_dirs(_tree: &ProjectTree, route: &RsReleaseRoute) -> BTreeSet<String> {
    route
        .family_files()
        .iter()
        .filter(|file| {
            file.kind() == guardrail3_app_rs_ownership::RustFamilyFileKind::CargoToml
                && (file.nearest_rust_root_rel().is_some()
                    || file.ancestor_rust_root_rels().is_some())
        })
        .map(|file| file.logical_owner_rel().to_owned())
        .collect()
}

fn expanded_workspace_members(tree: &ProjectTree, root: &CargoRootFacts) -> Vec<String> {
    root.workspace_members
        .iter()
        .flat_map(|pattern| expand_member_pattern(tree, &root.rel_dir, pattern))
        .collect()
}

fn expand_member_pattern(tree: &ProjectTree, workspace_rel: &str, pattern: &str) -> Vec<String> {
    let normalized = normalize_rel_dir(join_rel_dir("", pattern));
    let rel_pattern = if workspace_rel.is_empty() {
        normalized.clone()
    } else {
        normalize_rel_dir(join_rel_dir(workspace_rel, &normalized))
    };

    if looks_like_glob(&normalized) {
        tree.matching_dir_rels(&rel_pattern)
            .into_iter()
            .filter(|rel| tree.file_exists(&join_under_root(rel, "Cargo.toml")))
            .map(|rel| normalize_rel_dir(join_rel_dir("", &rel)))
            .collect()
    } else if tree.file_exists(&join_under_root(&rel_pattern, "Cargo.toml")) {
        vec![rel_pattern]
    } else {
        Vec::new()
    }
}

fn looks_like_glob(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?') || pattern.contains('[')
}

fn claimed_workspace_packages<'a>(
    tree: &ProjectTree,
    workspace_roots: impl Iterator<Item = &'a CargoRootFacts>,
) -> Vec<String> {
    let workspace_dirs = workspace_roots
        .filter(|root| root.has_workspace)
        .map(|root| root.rel_dir.clone())
        .collect::<BTreeSet<_>>();

    if workspace_dirs.is_empty() {
        return Vec::new();
    }

    let mut claimed = BTreeSet::new();
    if tree.file_exists("Cargo.toml") {
        let _ = maybe_collect_claimed_workspace_root(tree, "", &workspace_dirs, &mut claimed);
    }
    for dir in tree.dirs_with_file("Cargo.toml") {
        let _ = maybe_collect_claimed_workspace_root(tree, &dir, &workspace_dirs, &mut claimed);
    }

    claimed.into_iter().collect()
}

fn maybe_collect_claimed_workspace_root(
    tree: &ProjectTree,
    rel_dir: &str,
    workspace_dirs: &BTreeSet<String>,
    claimed: &mut BTreeSet<String>,
) -> Option<()> {
    if workspace_dirs.contains(rel_dir) {
        return Some(());
    }

    let cargo_rel_path = if rel_dir.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        join_under_root(rel_dir, "Cargo.toml")
    };
    let content = tree.file_content(&cargo_rel_path)?;
    let parsed = toml::from_str::<toml::Value>(content).ok()?;
    let package_workspace = parsed
        .get("package")
        .and_then(|package| package.get("workspace"))
        .and_then(toml::Value::as_str)?;
    let workspace_rel = normalize_rel_dir(join_rel_dir(rel_dir, package_workspace));
    if workspace_dirs.contains(&workspace_rel) {
        let _ = claimed.insert(rel_dir.to_owned());
    }

    Some(())
}

pub(super) fn workspace_root_for_package<'a>(
    root: &CargoRootFacts,
    cargo_roots: &'a BTreeMap<String, CargoRootFacts>,
) -> Option<&'a CargoRootFacts> {
    if let Some(workspace_ref) = root.package_workspace.as_deref() {
        let workspace_rel_dir = normalize_rel_dir(join_rel_dir(&root.rel_dir, workspace_ref));
        return cargo_roots.get(&workspace_rel_dir).filter(|candidate| {
            candidate.has_workspace && workspace_contains_package(candidate, root)
        });
    }

    cargo_roots
        .values()
        .filter(|candidate| candidate.has_workspace && workspace_contains_package(candidate, root))
        .max_by_key(|candidate| candidate.rel_dir.len())
}

fn workspace_contains_package(
    workspace_root: &CargoRootFacts,
    package_root: &CargoRootFacts,
) -> bool {
    if package_root.rel_dir == workspace_root.rel_dir {
        return true;
    }
    if workspace_root.workspace_exclude.iter().any(|pattern| {
        workspace_member_pattern_matches(workspace_root, pattern, &package_root.rel_dir)
    }) {
        return false;
    }
    workspace_root.workspace_members.iter().any(|pattern| {
        workspace_member_pattern_matches(workspace_root, pattern, &package_root.rel_dir)
    })
}

fn workspace_member_pattern_matches(
    workspace_root: &CargoRootFacts,
    pattern: &str,
    package_rel_dir: &str,
) -> bool {
    let repo_pattern = normalize_rel_dir(join_rel_dir(&workspace_root.rel_dir, pattern));
    Pattern::new(&repo_pattern)
        .map(|pattern| pattern.matches(package_rel_dir))
        .unwrap_or(false)
}

fn parse_root(
    tree: &ProjectTree,
    rel_dir: &str,
    input_failures: &mut Vec<ReleaseInputFailureFacts>,
) -> Option<CargoRootFacts> {
    let cargo_rel_path = if rel_dir.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        join_under_root(rel_dir, "Cargo.toml")
    };
    let Some(content) = tree.file_content(&cargo_rel_path) else {
        input_failures.push(ReleaseInputFailureFacts {
            rel_path: cargo_rel_path.clone(),
            message: "Failed to read Cargo.toml for release-family discovery.".to_owned(),
        });
        return None;
    };
    let parsed = match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => parsed,
        Err(parse_error) => {
            input_failures.push(ReleaseInputFailureFacts {
                rel_path: cargo_rel_path.clone(),
                message: format!(
                    "Failed to parse Cargo.toml for release-family discovery: {parse_error}"
                ),
            });
            return None;
        }
    };
    let workspace_dependencies = parsed
        .get("workspace")
        .and_then(|workspace| workspace.get("dependencies"))
        .and_then(toml::Value::as_table)
        .cloned()
        .unwrap_or_default();
    let workspace_members = parsed
        .get("workspace")
        .and_then(|workspace| workspace.get("members"))
        .and_then(toml::Value::as_array)
        .map(|members| {
            members
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let workspace_exclude = parsed
        .get("workspace")
        .and_then(|workspace| workspace.get("exclude"))
        .and_then(toml::Value::as_array)
        .map(|exclude| {
            exclude
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let package_workspace = parsed
        .get("package")
        .and_then(|package| package.get("workspace"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned);

    Some(CargoRootFacts {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path,
        has_workspace: parsed.get("workspace").is_some(),
        has_package: parsed.get("package").is_some(),
        workspace_members,
        workspace_exclude,
        workspace_dependencies,
        package_workspace,
        parsed,
    })
}

fn join_rel_dir(base_rel_dir: &str, rel: &str) -> PathBuf {
    if base_rel_dir.is_empty() {
        PathBuf::from(rel)
    } else {
        Path::new(base_rel_dir).join(rel)
    }
}

fn normalize_rel_dir(path: PathBuf) -> String {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = parts.pop();
            }
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::RootDir | Component::Prefix(_) => {}
        }
    }
    parts.join("/")
}
