use std::collections::{BTreeMap, BTreeSet};

use glob::Pattern;

use guardrail3_app_rs_family_mapper::RsDepsRoute;
use guardrail3_domain_project_tree::ProjectTree;

use super::guardrail::validate_workspace_manifest_shape;
use super::{InputFailureFacts, WorkspaceFacts};

pub(super) fn discover_workspaces(
    tree: &ProjectTree,
    route: &RsDepsRoute,
    exact_root_cargo_dirs: &BTreeSet<String>,
    input_failures: &mut Vec<InputFailureFacts>,
) -> Vec<WorkspaceFacts> {
    let mut roots = route
        .roots()
        .iter()
        .map(|root| root.rel_dir().to_owned())
        .collect::<Vec<_>>();
    if roots.is_empty() {
        roots.extend(exact_root_cargo_dirs.iter().cloned());
    }
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
            input_failures.push(InputFailureFacts {
                rel_path: cargo_rel_path.clone(),
                message: "Failed to read Cargo.toml for dependency root discovery.".to_owned(),
            });
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
            for member_dir in resolve_member_pattern(&root_rel_dir, member, exact_root_cargo_dirs) {
                let _ = member_dirs.insert(member_dir);
            }
            let direct_member = if root_rel_dir.is_empty() {
                member.trim_matches('/').to_owned()
            } else {
                format!("{root_rel_dir}/{}", member.trim_matches('/'))
            };
            let direct_member = direct_member.trim_matches('/').to_owned();
            if !direct_member.is_empty()
                && tree.file_exists(&format!("{direct_member}/Cargo.toml"))
            {
                let _ = member_dirs.insert(direct_member);
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
    workspace_root_rel_dir: &str,
    member: &str,
    exact_root_cargo_dirs: &BTreeSet<String>,
) -> Vec<String> {
    let pattern = if workspace_root_rel_dir.is_empty() {
        member.trim_matches('/').to_owned()
    } else {
        format!("{workspace_root_rel_dir}/{}", member.trim_matches('/'))
    };

    let matcher = Pattern::new(&pattern).ok();
    let mut matches = exact_root_cargo_dirs
        .iter()
        .filter(|candidate| matcher.as_ref().is_some_and(|pattern| pattern.matches(candidate)))
        .cloned()
        .collect::<Vec<_>>();
    if matches.is_empty() && exact_root_cargo_dirs.contains(&pattern) {
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
