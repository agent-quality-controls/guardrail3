use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsDepsRoute;
use guardrail3_domain_project_tree::ProjectTree;

use super::{InputFailureFacts, WorkspaceFacts};
use super::guardrail::validate_workspace_manifest_shape;

pub(super) fn discover_workspaces(
    tree: &ProjectTree,
    route: &RsDepsRoute,
    input_failures: &mut Vec<InputFailureFacts>,
) -> Vec<WorkspaceFacts> {
    let mut roots = route
        .roots
        .iter()
        .map(|root| root.rel_dir.clone())
        .collect::<Vec<_>>();
    roots.sort();
    roots.dedup();

    let mut workspaces = Vec::new();
    for root_rel_dir in roots {
        let cargo_rel_path = if root_rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{root_rel_dir}/Cargo.toml")
        };
        let Some(content) = tree.file_content(&cargo_rel_path) else {
            continue;
        };
        let parsed = match toml::from_str::<toml::Value>(content) {
            Ok(parsed) => parsed,
            Err(parse_error) => {
                input_failures.push(InputFailureFacts {
                    rel_path: cargo_rel_path.clone(),
                    message: format!("Failed to parse workspace Cargo.toml: {parse_error}"),
                });
                continue;
            }
        };
        if let Err(parse_error) = validate_workspace_manifest_shape(&parsed) {
            input_failures.push(InputFailureFacts {
                rel_path: cargo_rel_path.clone(),
                message: format!("Failed to parse workspace Cargo.toml: {parse_error}"),
            });
            continue;
        }
        let Some(workspace) = parsed.get("workspace") else {
            continue;
        };
        let raw_members = workspace
            .get("members")
            .and_then(toml::Value::as_array)
            .map(|members| {
                members
                    .iter()
                    .filter_map(toml::Value::as_str)
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let mut member_dirs = BTreeSet::new();
        for member in &raw_members {
            for member_dir in resolve_member_pattern(tree, &root_rel_dir, member) {
                if tree.file_exists(&format!("{member_dir}/Cargo.toml")) {
                    let _ = member_dirs.insert(member_dir);
                } else {
                    input_failures.push(InputFailureFacts {
                        rel_path: member_dir.clone(),
                        message: format!(
                            "Workspace member `{member_dir}` matched `{member}` but has no Cargo.toml."
                        ),
                    });
                }
            }
        }
        let workspace_dependencies = workspace
            .get("dependencies")
            .and_then(toml::Value::as_table)
            .cloned()
            .unwrap_or_default();
        let mut workspace_package_dirs = member_dirs.clone();
        if parsed.get("package").is_some() {
            let _ = workspace_package_dirs.insert(root_rel_dir.clone());
        }
        workspaces.push(WorkspaceFacts {
            root_rel_dir,
            cargo_rel_path,
            workspace_dependencies,
            workspace_package_dirs,
            member_dirs: member_dirs.into_iter().collect(),
        });
    }
    workspaces
}

pub(super) fn resolve_member_pattern(
    tree: &ProjectTree,
    workspace_root_rel_dir: &str,
    member: &str,
) -> Vec<String> {
    let pattern = if workspace_root_rel_dir.is_empty() {
        member.to_owned()
    } else {
        format!("{workspace_root_rel_dir}/{member}")
    };

    let mut matches = tree.matching_dir_rels(&pattern);
    if matches.is_empty() && tree.dir_exists(&pattern) {
        matches.push(pattern);
    }
    matches.sort();
    matches.dedup();
    matches
}

pub(super) fn workspace_by_member(workspaces: &[WorkspaceFacts]) -> BTreeMap<String, String> {
    let mut mapping = BTreeMap::new();
    let mut ordered = workspaces.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|workspace| std::cmp::Reverse(workspace.root_rel_dir.len()));
    for workspace in ordered {
        for member in &workspace.member_dirs {
            let _ = mapping
                .entry(member.clone())
                .or_insert_with(|| workspace.root_rel_dir.clone());
        }
    }
    mapping
}
