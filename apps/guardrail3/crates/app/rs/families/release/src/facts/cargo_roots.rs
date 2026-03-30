use std::collections::BTreeMap;
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
    route
        .roots()
        .iter()
        .map(|root| root.rel_dir().to_owned())
        .filter_map(|rel_dir| {
            let cargo_rel_path = if rel_dir.is_empty() {
                "Cargo.toml".to_owned()
            } else {
                join_under_root(&rel_dir, "Cargo.toml")
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
            Some((
                rel_dir.clone(),
                CargoRootFacts {
                    rel_dir,
                    cargo_rel_path,
                    has_workspace: parsed.get("workspace").is_some(),
                    has_package: parsed.get("package").is_some(),
                    workspace_members,
                    workspace_exclude,
                    workspace_dependencies,
                    package_workspace,
                    parsed,
                },
            ))
        })
        .collect()
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
