use std::collections::BTreeMap;

use guardrail3_app_rs_family_mapper::RsClippyRoute;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_domain_project_tree::ProjectTree;

use super::CargoRootFacts;

pub(super) fn collect_cargo_roots(
    tree: &ProjectTree,
    route: &RsClippyRoute,
    validation_scope: Option<&str>,
) -> BTreeMap<String, CargoRootFacts> {
    let routed_root_rels = route
        .roots()
        .iter()
        .map(|root| root.rel_dir())
        .collect::<std::collections::BTreeSet<_>>();

    route
        .family_files()
        .iter()
        .filter(|file| file.kind() == RustFamilyFileKind::CargoToml && file.exact_rust_root_owner())
        .filter(|file| {
            routed_root_rels.contains(file.logical_owner_rel())
                || route.family_files().iter().any(|candidate| {
                    matches!(
                        candidate.kind(),
                        RustFamilyFileKind::ClippyToml | RustFamilyFileKind::ClippyDotToml
                    ) && candidate.logical_owner_rel() == file.logical_owner_rel()
                })
        })
        .filter(|file| {
            validation_scope.is_none_or(|scope_rel| {
                path_is_under(file.logical_owner_rel(), scope_rel)
                    || path_is_under(scope_rel, file.logical_owner_rel())
            })
        })
        .map(|file| {
            (
                file.logical_owner_rel().to_owned(),
                file.rel_path().to_owned(),
            )
        })
        .map(|(rel_dir, cargo_rel_path)| {
            let facts = match tree.file_content(&cargo_rel_path) {
                Some(content) => match toml::from_str::<toml::Value>(content) {
                    Ok(parsed) => CargoRootFacts {
                        rel_dir: rel_dir.clone(),
                        cargo_rel_path: cargo_rel_path.clone(),
                        parse_error: None,
                        has_workspace: parsed.get("workspace").is_some(),
                        workspace_members: parse_workspace_members(tree, &rel_dir, &parsed),
                    },
                    Err(err) => CargoRootFacts {
                        rel_dir: rel_dir.clone(),
                        cargo_rel_path: cargo_rel_path.clone(),
                        parse_error: Some(err.to_string()),
                        has_workspace: false,
                        workspace_members: Vec::new(),
                    },
                },
                None => CargoRootFacts {
                    rel_dir: rel_dir.clone(),
                    cargo_rel_path: cargo_rel_path.clone(),
                    parse_error: Some("Cargo.toml content missing from ProjectTree".to_owned()),
                    has_workspace: false,
                    workspace_members: Vec::new(),
                },
            };
            (rel_dir, facts)
        })
        .collect()
}

fn path_is_under(rel_path: &str, parent_rel: &str) -> bool {
    parent_rel.is_empty()
        || rel_path == parent_rel
        || rel_path
            .strip_prefix(parent_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}

fn parse_workspace_members(
    tree: &ProjectTree,
    workspace_rel: &str,
    parsed: &toml::Value,
) -> Vec<String> {
    parsed
        .get("workspace")
        .and_then(|value| value.get("members"))
        .and_then(toml::Value::as_array)
        .map(|members| {
            members
                .iter()
                .filter_map(toml::Value::as_str)
                .flat_map(|member| expand_member_pattern(tree, workspace_rel, member))
                .collect()
        })
        .unwrap_or_default()
}

fn expand_member_pattern(tree: &ProjectTree, workspace_rel: &str, member: &str) -> Vec<String> {
    let trimmed = member.trim_matches('/');
    let pattern = if workspace_rel.is_empty() {
        trimmed.to_owned()
    } else {
        ProjectTree::join_rel(workspace_rel, trimmed)
    };

    if trimmed.contains('*') || trimmed.contains('?') || trimmed.contains('[') {
        tree.matching_dir_rels(&pattern)
    } else {
        vec![pattern]
    }
}
