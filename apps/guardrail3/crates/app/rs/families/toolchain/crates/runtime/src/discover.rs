use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsToolchainRoute;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_domain_project_tree::ProjectTree;

use super::facts::{
    AncestorToolchainFacts, DescendantToolchainFacts, ToolchainFamilyFacts,
    ToolchainPolicyRootFacts, UnownedToolchainFacts,
};

#[derive(Debug, Clone)]
struct CargoSnapshot {
    rel_dir: String,
    cargo_rel_path: String,
    parse_error: Option<String>,
    has_workspace: bool,
    rust_version: Option<String>,
    rust_version_invalid: bool,
}

pub fn collect(tree: &ProjectTree, route: &RsToolchainRoute) -> ToolchainFamilyFacts {
    let snapshots = collect_cargo_snapshots(tree, route);
    let workspace_roots: Vec<_> = snapshots
        .values()
        .filter(|snapshot| snapshot.has_workspace)
        .map(|snapshot| snapshot.rel_dir.clone())
        .collect();
    let mut policy_roots = Vec::new();

    for workspace_root_rel in &workspace_roots {
        let Some(snapshot) = snapshots.get(workspace_root_rel) else {
            continue;
        };
        policy_roots.push(build_policy_root(tree, route, snapshot));
    }

    policy_roots.sort_by(|a, b| a.cargo_rel_path.cmp(&b.cargo_rel_path));

    let unowned_toolchains = unowned_toolchains(&policy_roots, route);

    ToolchainFamilyFacts {
        policy_roots,
        unowned_toolchains,
    }
}

fn collect_cargo_snapshots(
    tree: &ProjectTree,
    route: &RsToolchainRoute,
) -> BTreeMap<String, CargoSnapshot> {
    route
        .roots()
        .iter()
        .map(|root| {
            let snapshot = cargo_snapshot(tree, root.rel_dir(), root.cargo_rel_path());
            (snapshot.rel_dir.clone(), snapshot)
        })
        .collect()
}

fn cargo_snapshot(tree: &ProjectTree, rel_dir: &str, cargo_rel_path: &str) -> CargoSnapshot {
    let Some(content) = tree.file_content(cargo_rel_path) else {
        return CargoSnapshot {
            rel_dir: rel_dir.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            parse_error: Some("Cargo.toml content missing from ProjectTree".to_owned()),
            has_workspace: false,
            rust_version: None,
            rust_version_invalid: false,
        };
    };

    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => {
            let has_workspace = parsed.get("workspace").is_some();
            let rust_version = extract_rust_version(&parsed, has_workspace);
            CargoSnapshot {
                rel_dir: rel_dir.to_owned(),
                cargo_rel_path: cargo_rel_path.to_owned(),
                parse_error: None,
                has_workspace,
                rust_version: rust_version.value,
                rust_version_invalid: rust_version.invalid,
            }
        }
        Err(parse_error) => CargoSnapshot {
            rel_dir: rel_dir.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            parse_error: Some(parse_error.to_string()),
            has_workspace: false,
            rust_version: None,
            rust_version_invalid: false,
        },
    }
}

fn build_policy_root(
    tree: &ProjectTree,
    route: &RsToolchainRoute,
    snapshot: &CargoSnapshot,
) -> ToolchainPolicyRootFacts {
    let modern_toolchain_rel = route.family_files().iter().find_map(|file| {
        (file.logical_owner_rel() == snapshot.rel_dir
            && file.exact_rust_root_owner()
            && file.kind() == RustFamilyFileKind::RustToolchainToml)
            .then(|| file.rel_path().to_owned())
    });
    let legacy_toolchain_rel = route.family_files().iter().find_map(|file| {
        (file.logical_owner_rel() == snapshot.rel_dir
            && file.exact_rust_root_owner()
            && file.kind() == RustFamilyFileKind::RustToolchainLegacy)
            .then(|| file.rel_path().to_owned())
    });

    let modern_toolchain = match modern_toolchain_rel.as_deref() {
        Some(toolchain_toml_rel) => match tree.file_content(toolchain_toml_rel) {
            Some(content) => match toml::from_str::<toml::Value>(content) {
                Ok(parsed) => (Some(toolchain_toml_rel.to_owned()), Some(parsed), None),
                Err(parse_error) => (
                    Some(toolchain_toml_rel.to_owned()),
                    None,
                    Some(parse_error.to_string()),
                ),
            },
            None => (
                Some(toolchain_toml_rel.to_owned()),
                None,
                Some("rust-toolchain.toml content missing from ProjectTree".to_owned()),
            ),
        },
        None => (None, None, None),
    };

    ToolchainPolicyRootFacts {
        rel_dir: snapshot.rel_dir.clone(),
        cargo_rel_path: snapshot.cargo_rel_path.clone(),
        toolchain_toml_rel: modern_toolchain.0,
        legacy_toolchain_rel,
        parsed: modern_toolchain.1,
        parse_error: modern_toolchain.2,
        cargo_rust_version: snapshot.rust_version.clone(),
        cargo_rust_version_invalid: snapshot.rust_version_invalid,
        cargo_parse_error: snapshot.parse_error.clone(),
        ancestor_toolchain: nearest_ancestor_toolchain(tree, route, &snapshot.rel_dir),
        descendant_toolchains: descendant_toolchains(route, &snapshot.rel_dir),
    }
}

struct RustVersionField {
    value: Option<String>,
    invalid: bool,
}

fn extract_rust_version(parsed: &toml::Value, is_workspace_root: bool) -> RustVersionField {
    if is_workspace_root {
        if let Some(value) = parsed
            .get("workspace")
            .and_then(|item| item.get("package"))
            .and_then(|item| item.get("rust-version"))
        {
            return RustVersionField {
                value: value.as_str().map(str::to_owned),
                invalid: !value.is_str(),
            };
        }
    }

    if let Some(value) = parsed
        .get("package")
        .and_then(|item| item.get("rust-version"))
    {
        return RustVersionField {
            value: value.as_str().map(str::to_owned),
            invalid: !value.is_str(),
        };
    }

    RustVersionField {
        value: None,
        invalid: false,
    }
}

fn is_nested_beneath(rel_dir: &str, parent_rel: &str) -> bool {
    if parent_rel.is_empty() {
        return !rel_dir.is_empty();
    }

    rel_dir
        .strip_prefix(parent_rel)
        .is_some_and(|suffix| suffix.starts_with('/'))
}

fn nearest_ancestor_toolchain(
    tree: &ProjectTree,
    route: &RsToolchainRoute,
    rel_dir: &str,
) -> Option<AncestorToolchainFacts> {
    for ancestor_rel in ancestor_rel_dirs(rel_dir) {
        if let Some(file) = route.family_files().iter().find(|file| {
            file.logical_owner_rel() == ancestor_rel
                && file.kind() == RustFamilyFileKind::RustToolchainLegacy
        }) {
            return Some(AncestorToolchainFacts {
                rel_path: file.rel_path().to_owned(),
                is_legacy: true,
                parsed: None,
                parse_error: None,
            });
        }

        let Some(file) = route.family_files().iter().find(|file| {
            file.logical_owner_rel() == ancestor_rel
                && file.kind() == RustFamilyFileKind::RustToolchainToml
        }) else {
            continue;
        };

        return Some(match tree.file_content(file.rel_path()) {
            Some(content) => match toml::from_str::<toml::Value>(content) {
                Ok(parsed) => AncestorToolchainFacts {
                    rel_path: file.rel_path().to_owned(),
                    is_legacy: false,
                    parsed: Some(parsed),
                    parse_error: None,
                },
                Err(parse_error) => AncestorToolchainFacts {
                    rel_path: file.rel_path().to_owned(),
                    is_legacy: false,
                    parsed: None,
                    parse_error: Some(parse_error.to_string()),
                },
            },
            None => AncestorToolchainFacts {
                rel_path: file.rel_path().to_owned(),
                is_legacy: false,
                parsed: None,
                parse_error: Some(
                    "ancestor rust-toolchain.toml content missing from ProjectTree".to_owned(),
                ),
            },
        });
    }

    None
}

fn ancestor_rel_dirs(rel_dir: &str) -> Vec<String> {
    let mut segments = rel_dir
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let mut ancestors = Vec::new();

    while !segments.is_empty() {
        let _ = segments.pop();
        ancestors.push(segments.join("/"));
    }

    ancestors
}

fn descendant_toolchains(route: &RsToolchainRoute, rel_dir: &str) -> Vec<DescendantToolchainFacts> {
    let mut descendants = route
        .family_files()
        .iter()
        .filter(|file| is_nested_beneath(file.logical_owner_rel(), rel_dir))
        .filter_map(|file| match file.kind() {
            RustFamilyFileKind::RustToolchainToml => Some(DescendantToolchainFacts {
                rel_path: file.rel_path().to_owned(),
                is_legacy: false,
            }),
            RustFamilyFileKind::RustToolchainLegacy => Some(DescendantToolchainFacts {
                rel_path: file.rel_path().to_owned(),
                is_legacy: true,
            }),
            _ => None,
        })
        .collect::<Vec<_>>();

    descendants.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    descendants
}

fn unowned_toolchains(
    policy_roots: &[ToolchainPolicyRootFacts],
    route: &RsToolchainRoute,
) -> Vec<UnownedToolchainFacts> {
    let allowed_roots = policy_roots
        .iter()
        .map(|root| root.rel_dir.clone())
        .collect::<BTreeSet<_>>();

    let mut toolchains = all_toolchain_files(route)
        .into_iter()
        .filter(|toolchain| {
            let rel_dir = toolchain
                .rel_path
                .rsplit_once('/')
                .map(|(dir, _)| dir)
                .unwrap_or("");

            !allowed_roots.contains(rel_dir)
                && !allowed_roots
                    .iter()
                    .any(|workspace_root| is_nested_beneath(rel_dir, workspace_root))
        })
        .collect::<Vec<_>>();
    toolchains.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    toolchains
}

fn all_toolchain_files(route: &RsToolchainRoute) -> Vec<UnownedToolchainFacts> {
    route
        .family_files()
        .iter()
        .filter_map(|file| match file.kind() {
            RustFamilyFileKind::RustToolchainToml => Some(UnownedToolchainFacts {
                rel_path: file.rel_path().to_owned(),
                is_legacy: false,
            }),
            RustFamilyFileKind::RustToolchainLegacy => Some(UnownedToolchainFacts {
                rel_path: file.rel_path().to_owned(),
                is_legacy: true,
            }),
            _ => None,
        })
        .collect()
}
