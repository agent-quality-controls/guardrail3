use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

use guardrail3_domain_project_tree::ProjectTree;

use super::{
    DependencyEntryFacts, DependencySectionKind, InputFailureFacts, MemberFacts, ParsedGuardrail,
    WorkspaceFacts,
};
use super::guardrail::validate_dependency_manifest_shape;

pub(super) fn discover_members(
    tree: &ProjectTree,
    routed_root_rels: &BTreeSet<String>,
    workspaces: &[WorkspaceFacts],
    workspace_by_member: &BTreeMap<String, String>,
    parsed_guardrail: &Option<ParsedGuardrail>,
    input_failures: &mut Vec<InputFailureFacts>,
) -> Vec<MemberFacts> {
    let mut member_dirs = workspaces
        .iter()
        .flat_map(|workspace| workspace.member_dirs.iter().cloned())
        .collect::<BTreeSet<_>>();

    for root_rel_dir in routed_root_rels.iter().cloned() {
        if workspace_by_member.contains_key(&root_rel_dir) {
            continue;
        }
        let cargo_rel_path = if root_rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{root_rel_dir}/Cargo.toml")
        };
        let Some(content) = tree.file_content(&cargo_rel_path) else {
            if tree.file_exists(&cargo_rel_path) {
                input_failures.push(InputFailureFacts {
                    rel_path: cargo_rel_path.clone(),
                    message: "Failed to read Cargo.toml for dependency root discovery.".to_owned(),
                });
            }
            continue;
        };
        let parsed = match toml::from_str::<toml::Value>(content) {
            Ok(parsed) => parsed,
            Err(parse_error) => {
                input_failures.push(InputFailureFacts {
                    rel_path: cargo_rel_path.clone(),
                    message: format!(
                        "Failed to parse Cargo.toml for dependency root discovery: {parse_error}"
                    ),
                });
                continue;
            }
        };
        if parsed.get("package").is_some() {
            let _ = member_dirs.insert(root_rel_dir);
        }
    }

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

pub(super) fn policy_for_member(
    rel_dir: &str,
    parsed_guardrail: Option<&ParsedGuardrail>,
) -> (Option<String>, Option<BTreeSet<String>>) {
    let Some(parsed_guardrail) = parsed_guardrail else {
        return (None, None);
    };

    if rel_dir.starts_with("apps/") {
        if let Some(app_name) = rel_dir.split('/').nth(1) {
            if let Some(config) = parsed_guardrail.apps.get(app_name) {
                return (
                    config.profile.clone().or(config.type_.clone()),
                    config
                        .allowed_deps
                        .clone()
                        .map(|deps| deps.into_iter().collect::<BTreeSet<_>>()),
                );
            }
        }
    }

    if rel_dir.starts_with("packages/") {
        if let Some(config) = &parsed_guardrail.packages {
            return (
                config.profile.clone().or(config.type_.clone()),
                config
                    .allowed_deps
                    .clone()
                    .map(|deps| deps.into_iter().collect::<BTreeSet<_>>()),
            );
        }
    }

    (parsed_guardrail.root_profile_name.clone(), None)
}

pub(super) fn collect_dependency_entries(
    tree: &ProjectTree,
    members: &[MemberFacts],
    workspaces: &[WorkspaceFacts],
    input_failures: &mut Vec<InputFailureFacts>,
) -> Vec<DependencyEntryFacts> {
    let workspaces_by_root = workspaces
        .iter()
        .map(|workspace| (workspace.root_rel_dir.clone(), workspace))
        .collect::<BTreeMap<_, _>>();
    let mut entries = Vec::new();

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
        if let Err(parse_error) = validate_dependency_manifest_shape(&parsed) {
            input_failures.push(InputFailureFacts {
                rel_path: member.cargo_rel_path.clone(),
                message: format!(
                    "Failed to parse Cargo.toml for dependency policy check: {parse_error}"
                ),
            });
            continue;
        }

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
                match external_dep_name(member, alias, value, &workspaces_by_root) {
                    Ok(Some(dep_package_name)) => entries.push(DependencyEntryFacts {
                        crate_name: member.crate_name.clone(),
                        cargo_rel_path: member.cargo_rel_path.clone(),
                        section_kind,
                        dep_alias: alias.clone(),
                        allowlisted: member
                            .allowed_deps
                            .as_ref()
                            .is_some_and(|allowed| allowed.contains(&dep_package_name)),
                        allowlist_present: member.allowed_deps.is_some(),
                        dep_package_name,
                    }),
                    Ok(None) => {}
                    Err(message) => input_failures.push(InputFailureFacts {
                        rel_path: member.cargo_rel_path.clone(),
                        message,
                    }),
                }
            }
        }
    }

    entries
}

fn external_dep_name(
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
            if is_workspace_package_path(&workspace.root_rel_dir, dep_path, workspace) {
                return Ok(None);
            }
        }
        return Ok(Some(workspace_package));
    }

    if let Some(dep_path) = dep_table
        .and_then(|table| table.get("path"))
        .and_then(toml::Value::as_str)
    {
        if let Some(workspace_root) = &member.workspace_root_rel_dir {
            if let Some(workspace) = workspaces_by_root.get(workspace_root) {
                if is_workspace_package_path(&member.rel_dir, dep_path, workspace) {
                    return Ok(None);
                }
            }
        }
    }

    Ok(Some(package_name))
}

fn is_workspace_package_path(
    base_rel_dir: &str,
    dep_path: &str,
    workspace: &WorkspaceFacts,
) -> bool {
    let resolved = normalize_rel_path(base_rel_dir, dep_path);
    workspace.workspace_package_dirs.contains(&resolved)
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
