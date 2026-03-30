use std::collections::BTreeMap;

use guardrail3_app_rs_family_mapper::RsClippyRoute;
use guardrail3_domain_project_tree::ProjectTree;

use super::CargoRootFacts;

pub(super) fn collect_cargo_roots(
    tree: &ProjectTree,
    route: &RsClippyRoute,
) -> BTreeMap<String, CargoRootFacts> {
    route
        .roots()
        .iter()
        .map(|root| (root.rel_dir().to_owned(), root.cargo_rel_path().to_owned()))
        .map(|(rel_dir, cargo_rel_path)| {
            let facts = match tree.file_content(&cargo_rel_path) {
                Some(content) => match toml::from_str::<toml::Value>(content) {
                    Ok(parsed) => CargoRootFacts {
                        rel_dir: rel_dir.clone(),
                        cargo_rel_path: cargo_rel_path.clone(),
                        parse_error: None,
                        has_workspace: parsed.get("workspace").is_some(),
                        has_package: parsed.get("package").is_some(),
                        workspace_members: parse_workspace_members(tree, &rel_dir, &parsed),
                    },
                    Err(err) => CargoRootFacts {
                        rel_dir: rel_dir.clone(),
                        cargo_rel_path: cargo_rel_path.clone(),
                        parse_error: Some(err.to_string()),
                        has_workspace: false,
                        has_package: false,
                        workspace_members: Vec::new(),
                    },
                },
                None => CargoRootFacts {
                    rel_dir: rel_dir.clone(),
                    cargo_rel_path: cargo_rel_path.clone(),
                    parse_error: Some("Cargo.toml content missing from ProjectTree".to_owned()),
                    has_workspace: false,
                    has_package: false,
                    workspace_members: Vec::new(),
                },
            };
            (rel_dir, facts)
        })
        .collect()
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
