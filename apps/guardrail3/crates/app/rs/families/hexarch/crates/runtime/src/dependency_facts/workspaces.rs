use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;

use super::{
    PatchEntryFacts, WorkspaceFacts, dir_is_within_owned_hex_scope, layer_from_path, normalize_path,
};

pub(super) fn discover_workspaces(
    tree: &ProjectTree,
    owned_app_roots: &BTreeSet<String>,
    include_repo_root_workspace: bool,
) -> Vec<WorkspaceFacts> {
    let mut workspaces = Vec::new();
    let mut seen = BTreeSet::new();

    let cargo_dirs = tree
        .structure()
        .iter()
        .filter_map(|(dir_rel, entry)| {
            if dir_rel.is_empty() || !entry.has_file("Cargo.toml") {
                return None;
            }
            Some(dir_rel.clone())
        })
        .collect::<Vec<String>>();
    let root_dirs = include_repo_root_workspace
        .then_some(String::new())
        .into_iter();
    for dir in root_dirs.chain(cargo_dirs) {
        if !seen.insert(dir.clone()) {
            continue;
        }
        if !dir.is_empty() && !dir_is_within_owned_hex_scope(&dir, owned_app_roots) {
            continue;
        }
        let cargo_rel_path = if dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{dir}/Cargo.toml")
        };
        let Some(content) = tree.file_content(&cargo_rel_path) else {
            continue;
        };
        let Ok(parsed) = toml::from_str::<toml::Value>(content) else {
            continue;
        };
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
        let member_dirs = raw_members
            .iter()
            .flat_map(|member| resolve_member_pattern(tree, &dir, member))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let workspace_dependencies = parsed
            .get("workspace")
            .and_then(|value| value.get("dependencies"))
            .and_then(toml::Value::as_table)
            .cloned()
            .unwrap_or_default();
        let patch_entries = parse_patch_entries(tree, &parsed, &dir, &cargo_rel_path);

        workspaces.push(WorkspaceFacts {
            root_rel_dir: dir,
            member_dirs,
            workspace_dependencies,
            patch_entries,
        });
    }

    workspaces.sort_by(|left, right| left.root_rel_dir.cmp(&right.root_rel_dir));
    workspaces
}

fn resolve_member_pattern(
    tree: &ProjectTree,
    workspace_root_rel_dir: &str,
    member: &str,
) -> Vec<String> {
    let pattern = if workspace_root_rel_dir.is_empty() {
        member.to_owned()
    } else {
        format!("{workspace_root_rel_dir}/{member}")
    };

    let mut matches = tree
        .matching_dir_rels(&pattern)
        .into_iter()
        .filter(|dir| tree.file_exists(&format!("{dir}/Cargo.toml")))
        .collect::<Vec<_>>();

    if matches.is_empty() && tree.file_exists(&format!("{pattern}/Cargo.toml")) {
        matches.push(pattern);
    }
    matches
}

fn parse_patch_entries(
    tree: &ProjectTree,
    parsed: &toml::Value,
    workspace_root_rel_dir: &str,
    cargo_rel_path: &str,
) -> Vec<PatchEntryFacts> {
    let mut patches = Vec::new();
    if let Some(patch_table) = parsed.get("patch").and_then(toml::Value::as_table) {
        for registry_table in patch_table.values().filter_map(toml::Value::as_table) {
            for (key, value) in registry_table {
                if let Some(path_value) = extract_path(value) {
                    let resolved = normalize_path(workspace_root_rel_dir, &path_value);
                    let target_layer = tree
                        .file_exists(&ProjectTree::join_rel(&resolved, "Cargo.toml"))
                        .then(|| layer_from_path(&resolved))
                        .flatten();
                    patches.push(PatchEntryFacts {
                        cargo_rel_path: cargo_rel_path.to_owned(),
                        key: key.clone(),
                        target_layer,
                        resolved_rel_dir: resolved,
                        escape_hatch_reason: None,
                    });
                }
            }
        }
    }

    if let Some(replace_table) = parsed.get("replace").and_then(toml::Value::as_table) {
        for (key, value) in replace_table {
            if let Some(path_value) = extract_path(value) {
                let resolved = normalize_path(workspace_root_rel_dir, &path_value);
                let target_layer = tree
                    .file_exists(&ProjectTree::join_rel(&resolved, "Cargo.toml"))
                    .then(|| layer_from_path(&resolved))
                    .flatten();
                patches.push(PatchEntryFacts {
                    cargo_rel_path: cargo_rel_path.to_owned(),
                    key: key.clone(),
                    target_layer,
                    resolved_rel_dir: resolved,
                    escape_hatch_reason: None,
                });
            }
        }
    }

    patches
}

pub(super) fn best_workspace_for_member(workspaces: &[WorkspaceFacts]) -> BTreeMap<String, String> {
    let mut mapping = BTreeMap::new();
    let mut ordered = workspaces.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|workspace| std::cmp::Reverse(workspace.root_rel_dir.len()));
    for workspace in ordered {
        for member_dir in &workspace.member_dirs {
            let _ = mapping
                .entry(member_dir.clone())
                .or_insert_with(|| workspace.root_rel_dir.clone());
        }
    }
    mapping
}

fn extract_path(value: &toml::Value) -> Option<String> {
    value
        .as_table()
        .and_then(|table| table.get("path"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
}
