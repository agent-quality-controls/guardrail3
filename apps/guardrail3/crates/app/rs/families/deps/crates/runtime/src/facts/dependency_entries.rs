use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

use guardrail3_domain_project_tree::ProjectTree;

use super::guardrail::{
    validate_target_dependency_manifest_shape, validate_top_level_dependency_manifest_shape,
};
use super::{
    DependencyEntryFacts, DependencySectionKind, DirectDependencyCapFacts, InputFailureFacts,
    MemberFacts, ParsedGuardrail, WorkspaceFacts,
};

pub(super) fn discover_members(
    tree: &ProjectTree,
    workspaces: &[WorkspaceFacts],
    workspace_by_member: &BTreeMap<String, String>,
    parsed_guardrail: &Option<ParsedGuardrail>,
) -> Vec<MemberFacts> {
    let member_dirs = workspaces
        .iter()
        .flat_map(|workspace| workspace.workspace_package_dirs.iter())
        .cloned()
        .collect::<BTreeSet<_>>();

    let mut members = Vec::new();
    for rel_dir in member_dirs {
        let cargo_rel_path = if rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{rel_dir}/Cargo.toml")
        };
        let crate_name = tree
            .file_content(&cargo_rel_path)
            .and_then(|content| toml::from_str::<toml::Value>(content).ok())
            .and_then(|parsed| {
                parsed
                    .get("package")
                    .and_then(|value| value.get("name"))
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| {
                if rel_dir.is_empty() {
                    "root".to_owned()
                } else {
                    rel_dir.rsplit('/').next().unwrap_or(&rel_dir).to_owned()
                }
            });
        let (profile_name, allowed_deps) = policy_for_member(&rel_dir, parsed_guardrail.as_ref());

        members.push(MemberFacts {
            crate_name,
            rel_dir: rel_dir.clone(),
            cargo_rel_path,
            workspace_root_rel_dir: {
                if let Some(workspace_root) = workspace_by_member.get(&rel_dir) {
                    Some(workspace_root.clone())
                } else if workspaces
                    .iter()
                    .any(|workspace| workspace.root_rel_dir == rel_dir)
                {
                    Some(rel_dir.clone())
                } else {
                    None
                }
            },
            profile_name,
            allowed_deps,
        });
    }
    members.sort_by(|left, right| left.rel_dir.cmp(&right.rel_dir));
    members
}

fn path_is_under(rel_path: &str, parent_rel: &str) -> bool {
    parent_rel.is_empty()
        || rel_path == parent_rel
        || rel_path
            .strip_prefix(parent_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}

pub(super) fn policy_for_member(
    rel_dir: &str,
    parsed_guardrail: Option<&ParsedGuardrail>,
) -> (Option<String>, Option<BTreeSet<String>>) {
    let Some(parsed_guardrail) = parsed_guardrail else {
        return (None, None);
    };

    match governed_zone_scope(rel_dir) {
        Some(GovernedZoneScope::App(app_name)) => {
            if let Some(config) = parsed_guardrail.apps.get(app_name) {
                return (
                    config
                        .profile_name
                        .clone()
                        .or_else(|| config.type_name.clone()),
                    config.allowed_deps.clone(),
                );
            }
        }
        Some(GovernedZoneScope::Packages) => {
            if let Some(config) = &parsed_guardrail.packages {
                return (
                    config
                        .profile_name
                        .clone()
                        .or_else(|| config.type_name.clone()),
                    config.allowed_deps.clone(),
                );
            }
        }
        None => {}
    }

    (parsed_guardrail.root_profile_name.clone(), None)
}

enum GovernedZoneScope<'a> {
    App(&'a str),
    Packages,
}

fn governed_zone_scope(rel_dir: &str) -> Option<GovernedZoneScope<'_>> {
    let segments = rel_dir
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    if segments.len() < 2 {
        return None;
    }

    let mut app_names = Vec::new();
    let mut package_hits = 0usize;
    for window in segments.windows(2) {
        match window {
            ["apps", app_name] => app_names.push(*app_name),
            ["packages", _] => package_hits += 1,
            _ => {}
        }
    }

    match (app_names.len(), package_hits) {
        (1, 0) => Some(GovernedZoneScope::App(app_names[0])),
        (0, 1) => Some(GovernedZoneScope::Packages),
        _ => None,
    }
}

pub(super) fn collect_dependency_facts(
    tree: &ProjectTree,
    members: &[MemberFacts],
    workspaces: &[WorkspaceFacts],
    input_failures: &mut Vec<InputFailureFacts>,
) -> (Vec<DependencyEntryFacts>, Vec<DirectDependencyCapFacts>) {
    let workspaces_by_root = workspaces
        .iter()
        .map(|workspace| (workspace.root_rel_dir.clone(), workspace))
        .collect::<BTreeMap<_, _>>();
    let mut entries = Vec::new();
    let mut direct_dependency_caps = Vec::new();

    for member in members {
        let Some(content) = tree.file_content(&member.cargo_rel_path) else {
            input_failures.push(InputFailureFacts {
                rel_path: member.cargo_rel_path.clone(),
                message: "Missing Cargo.toml content for dependency policy check.".to_owned(),
            });
            continue;
        };
        let parsed = match toml::from_str::<toml::Value>(content) {
            Ok(parsed) => parsed,
            Err(parse_error) => {
                input_failures.push(InputFailureFacts {
                    rel_path: member.cargo_rel_path.clone(),
                    message: format!(
                        "Failed to parse Cargo.toml for dependency policy check: {parse_error}"
                    ),
                });
                continue;
            }
        };
        if let Err(parse_error) = validate_top_level_dependency_manifest_shape(&parsed) {
            input_failures.push(InputFailureFacts {
                rel_path: member.cargo_rel_path.clone(),
                message: format!(
                    "Failed to parse Cargo.toml for dependency policy check: {parse_error}"
                ),
            });
            continue;
        }

        let mut unique_direct_dependency_names = BTreeSet::new();
        let mut saw_error = collect_top_level_dependency_entries(
            tree,
            member,
            &parsed,
            &workspaces_by_root,
            input_failures,
            &mut entries,
            &mut unique_direct_dependency_names,
        );
        if let Err(parse_error) = validate_target_dependency_manifest_shape(&parsed) {
            input_failures.push(InputFailureFacts {
                rel_path: member.cargo_rel_path.clone(),
                message: format!(
                    "Failed to parse Cargo.toml for dependency policy check: {parse_error}"
                ),
            });
            saw_error = true;
        } else {
            saw_error |= collect_target_dependency_entries(
                tree,
                member,
                &parsed,
                &workspaces_by_root,
                input_failures,
                &mut entries,
                &mut unique_direct_dependency_names,
            );
        }
        if !saw_error {
            direct_dependency_caps.push(DirectDependencyCapFacts {
                crate_name: member.crate_name.clone(),
                cargo_rel_path: member.cargo_rel_path.clone(),
                unique_direct_dependency_count: unique_direct_dependency_names.len(),
            });
        }
    }

    (entries, direct_dependency_caps)
}

fn collect_top_level_dependency_entries(
    tree: &ProjectTree,
    member: &MemberFacts,
    parsed: &toml::Value,
    workspaces_by_root: &BTreeMap<String, &WorkspaceFacts>,
    input_failures: &mut Vec<InputFailureFacts>,
    entries: &mut Vec<DependencyEntryFacts>,
    direct_dependency_names: &mut BTreeSet<String>,
) -> bool {
    let mut saw_error = false;
    for (section_key, section_kind) in [
        ("dependencies", DependencySectionKind::Dependencies),
        (
            "build-dependencies",
            DependencySectionKind::BuildDependencies,
        ),
        ("dev-dependencies", DependencySectionKind::DevDependencies),
    ] {
        let Some(table) = parsed.get(section_key).and_then(toml::Value::as_table) else {
            continue;
        };
        for (alias, value) in table {
            match external_dep_name(tree, member, alias, value, workspaces_by_root) {
                Ok(Some(dep_package_name)) => {
                    let _ = direct_dependency_names.insert(dep_package_name.clone());
                    entries.push(DependencyEntryFacts {
                        crate_name: member.crate_name.clone(),
                        cargo_rel_path: member.cargo_rel_path.clone(),
                        section_kind,
                        table_label: format!("[{section_key}]"),
                        allowlisted: member
                            .allowed_deps
                            .as_ref()
                            .is_some_and(|allowed| allowed.contains(&dep_package_name)),
                        allowlist_present: member.allowed_deps.is_some(),
                        dep_package_name,
                    });
                }
                Ok(None) => {}
                Err(message) => {
                    input_failures.push(InputFailureFacts {
                        rel_path: member.cargo_rel_path.clone(),
                        message,
                    });
                    saw_error = true;
                }
            }
        }
    }
    saw_error
}

fn collect_target_dependency_entries(
    tree: &ProjectTree,
    member: &MemberFacts,
    parsed: &toml::Value,
    workspaces_by_root: &BTreeMap<String, &WorkspaceFacts>,
    input_failures: &mut Vec<InputFailureFacts>,
    entries: &mut Vec<DependencyEntryFacts>,
    names: &mut BTreeSet<String>,
) -> bool {
    let mut saw_error = false;

    if let Some(targets) = parsed.get("target").and_then(toml::Value::as_table) {
        for (target_key, target_value) in targets {
            let Some(target_table) = target_value.as_table() else {
                continue;
            };
            for (section_key, section_kind) in [
                ("dependencies", DependencySectionKind::Dependencies),
                (
                    "build-dependencies",
                    DependencySectionKind::BuildDependencies,
                ),
                ("dev-dependencies", DependencySectionKind::DevDependencies),
            ] {
                let Some(table) = target_table
                    .get(section_key)
                    .and_then(toml::Value::as_table)
                else {
                    continue;
                };
                saw_error |= collect_dependency_entries_from_table(
                    tree,
                    member,
                    table,
                    workspaces_by_root,
                    input_failures,
                    Some((section_kind, format!("[target.'{target_key}'.{section_key}]"))),
                    entries,
                    names,
                );
            }
        }
    }

    saw_error
}

fn collect_dependency_entries_from_table(
    tree: &ProjectTree,
    member: &MemberFacts,
    table: &toml::map::Map<String, toml::Value>,
    workspaces_by_root: &BTreeMap<String, &WorkspaceFacts>,
    input_failures: &mut Vec<InputFailureFacts>,
    section_details: Option<(DependencySectionKind, String)>,
    entries: &mut Vec<DependencyEntryFacts>,
    names: &mut BTreeSet<String>,
) -> bool {
    let mut saw_error = false;
    for (alias, value) in table {
        match external_dep_name(tree, member, alias, value, workspaces_by_root) {
            Ok(Some(dep_package_name)) => {
                let _ = names.insert(dep_package_name.clone());
                if let Some((section_kind, table_label)) = &section_details {
                    entries.push(DependencyEntryFacts {
                        crate_name: member.crate_name.clone(),
                        cargo_rel_path: member.cargo_rel_path.clone(),
                        section_kind: *section_kind,
                        table_label: table_label.clone(),
                        allowlisted: member
                            .allowed_deps
                            .as_ref()
                            .is_some_and(|allowed| allowed.contains(&dep_package_name)),
                        allowlist_present: member.allowed_deps.is_some(),
                        dep_package_name,
                    });
                }
            }
            Ok(None) => {}
            Err(message) => {
                input_failures.push(InputFailureFacts {
                    rel_path: member.cargo_rel_path.clone(),
                    message,
                });
                saw_error = true;
            }
        }
    }
    saw_error
}

fn external_dep_name(
    tree: &ProjectTree,
    member: &MemberFacts,
    alias: &str,
    value: &toml::Value,
    workspaces_by_root: &BTreeMap<String, &WorkspaceFacts>,
) -> Result<Option<String>, String> {
    let dep_table = value.as_table();
    let package_name = dep_table
        .and_then(|table| table.get("package"))
        .and_then(toml::Value::as_str)
        .unwrap_or(alias)
        .to_owned();

    let uses_workspace = dep_table
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        == Some(true);

    if uses_workspace {
        let workspace_root = member.workspace_root_rel_dir.as_ref().ok_or_else(|| {
            format!(
                "`{alias}` uses `workspace = true` but `{}` is not associated with a workspace root.",
                member.rel_dir
            )
        })?;
        let workspace = workspaces_by_root.get(workspace_root).ok_or_else(|| {
            format!(
                "Workspace dependency resolution failed for `{alias}` in `{}`: missing workspace facts.",
                member.rel_dir
            )
        })?;
        let workspace_value = workspace.workspace_dependencies.get(alias).ok_or_else(|| {
            format!(
                "`{alias}` uses `workspace = true` in `{}`, but `[workspace.dependencies].{alias}` is missing in `{}`.",
                member.cargo_rel_path, workspace.cargo_rel_path
            )
        })?;
        let workspace_package = workspace_value
            .as_table()
            .and_then(|table| table.get("package"))
            .and_then(toml::Value::as_str)
            .unwrap_or(alias)
            .to_owned();
        if let Some(dep_path) = workspace_value
            .as_table()
            .and_then(|table| table.get("path"))
            .and_then(toml::Value::as_str)
        {
            return resolve_path_dependency_identity(
                tree,
                alias,
                &member.cargo_rel_path,
                &workspace.root_rel_dir,
                dep_path,
                Some(workspace),
                workspace_package,
            );
        }
        return Ok(Some(workspace_package));
    }

    if let Some(dep_path) = dep_table
        .and_then(|table| table.get("path"))
        .and_then(toml::Value::as_str)
    {
        if let Some(workspace_root) = &member.workspace_root_rel_dir {
            if let Some(workspace) = workspaces_by_root.get(workspace_root) {
                return resolve_path_dependency_identity(
                    tree,
                    alias,
                    &member.cargo_rel_path,
                    &member.rel_dir,
                    dep_path,
                    Some(workspace),
                    package_name,
                );
            }
        }
        return resolve_path_dependency_identity(
            tree,
            alias,
            &member.cargo_rel_path,
            &member.rel_dir,
            dep_path,
            None,
            package_name,
        );
    }

    Ok(Some(package_name))
}

fn resolve_path_dependency_identity(
    tree: &ProjectTree,
    alias: &str,
    member_cargo_rel_path: &str,
    base_rel_dir: &str,
    dep_path: &str,
    workspace: Option<&WorkspaceFacts>,
    fallback_package_name: String,
) -> Result<Option<String>, String> {
    let resolved = normalize_rel_path(base_rel_dir, dep_path);

    if workspace.is_some_and(|workspace| workspace.workspace_package_dirs.contains(&resolved)) {
        return Ok(None);
    }

    let target_cargo_rel_path = if resolved.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        format!("{resolved}/Cargo.toml")
    };
    let target_is_local_cargo_package = tree.file_exists(&target_cargo_rel_path);

    if let Some(workspace) = workspace {
        if target_is_local_cargo_package && path_is_under(&resolved, &workspace.root_rel_dir) {
            return Err(format!(
                "`{alias}` in `{member_cargo_rel_path}` points to local Cargo package `{resolved}` under workspace root `{}` but that package is not declared in `[workspace].members`.",
                workspace.root_rel_dir
            ));
        }
    }

    if target_is_local_cargo_package {
        return read_local_package_name(tree, &target_cargo_rel_path).map(Some);
    }

    Ok(Some(fallback_package_name))
}

fn normalize_rel_path(base_rel_dir: &str, dep_path: &str) -> String {
    let joined = if base_rel_dir.is_empty() {
        Path::new(dep_path).to_path_buf()
    } else {
        Path::new(base_rel_dir).join(dep_path)
    };
    let mut parts = Vec::new();

    for component in joined.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::ParentDir => {
                if parts.last().is_some_and(|last| last != "..") {
                    let _ = parts.pop();
                } else {
                    parts.push("..".to_owned());
                }
            }
            Component::RootDir | Component::Prefix(_) => {}
        }
    }

    parts.join("/")
}

fn read_local_package_name(tree: &ProjectTree, cargo_rel_path: &str) -> Result<String, String> {
    let Some(content) = tree.file_content(cargo_rel_path) else {
        return Err(format!(
            "Failed to read local path dependency manifest `{cargo_rel_path}` for dependency policy resolution."
        ));
    };
    let parsed = toml::from_str::<toml::Value>(content).map_err(|parse_error| {
        format!(
            "Failed to parse local path dependency manifest `{cargo_rel_path}` for dependency policy resolution: {parse_error}"
        )
    })?;
    parsed
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| {
            format!(
                "Local path dependency manifest `{cargo_rel_path}` is missing `package.name` for dependency policy resolution."
            )
        })
}
