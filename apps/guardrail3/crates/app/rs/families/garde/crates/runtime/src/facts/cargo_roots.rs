use std::collections::BTreeMap;

use guardrail3_app_rs_family_mapper::RsGardeRoute;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use super::{CargoRootFacts, GardeInputFailureFacts};

pub(super) fn collect_cargo_roots(
    tree: &ProjectTree,
    route: &RsGardeRoute,
    input_failures: &mut Vec<GardeInputFailureFacts>,
) -> BTreeMap<String, CargoRootFacts> {
    route
        .family_files()
        .iter()
        .filter(|file| file.kind() == RustFamilyFileKind::CargoToml)
        .map(|file| {
            let rel_dir = file.logical_owner_rel().to_owned();
            let rel_path = file.rel_path().to_owned();
            let parsed = tree
                .file_content(&rel_path)
                .map(|content| toml::from_str::<toml::Value>(content));
            let facts = match parsed {
                Some(Ok(parsed)) => CargoRootFacts {
                    rel_dir: rel_dir.clone(),
                    has_workspace: parsed.get("workspace").is_some(),
                    workspace_members: parse_workspace_members(tree, &rel_dir, &parsed),
                },
                Some(Err(parse_error)) => {
                    input_failures.push(GardeInputFailureFacts {
                        rel_path: rel_path.clone(),
                        message: format!(
                            "Failed to parse Cargo.toml for garde root discovery: {parse_error}"
                        ),
                    });
                    CargoRootFacts {
                        rel_dir: rel_dir.clone(),
                        has_workspace: false,
                        workspace_members: Vec::new(),
                    }
                }
                None => CargoRootFacts {
                    rel_dir: rel_dir.clone(),
                    has_workspace: false,
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
