use std::collections::BTreeMap;

use guardrail3_domain_project_tree::ProjectTree;

use super::{
    DependencyEdgeFacts, EdgeKind, MemberDependencyFacts, WorkspaceFacts, app_root_for_dir,
    layer_from_path, normalize_path,
};

pub(super) fn collect_edges(
    tree: &ProjectTree,
    workspaces: &[WorkspaceFacts],
    members: &[MemberDependencyFacts],
    member_by_dir: &BTreeMap<String, &MemberDependencyFacts>,
) -> Vec<DependencyEdgeFacts> {
    let workspaces_by_root = workspaces
        .iter()
        .map(|workspace| (workspace.root_rel_dir.clone(), workspace))
        .collect::<BTreeMap<_, _>>();
    let mut edges = Vec::new();
    for member in members {
        if member.cargo_parse_error.is_some() {
            continue;
        }
        let Some(content) = tree.file_content(&member.cargo_rel_path) else {
            continue;
        };
        let Ok(parsed) = toml::from_str::<toml::Value>(content) else {
            continue;
        };
        let workspace_deps = member
            .workspace_root_rel_dir
            .as_ref()
            .and_then(|root| workspaces_by_root.get(root))
            .map(|workspace| &workspace.workspace_dependencies)
            .cloned()
            .unwrap_or_default();

        collect_edge_section(
            tree,
            &mut edges,
            member,
            member_by_dir,
            member.workspace_root_rel_dir.as_deref(),
            &workspace_deps,
            parsed.get("dependencies"),
            EdgeKind::Dependency,
        );
        collect_edge_section(
            tree,
            &mut edges,
            member,
            member_by_dir,
            member.workspace_root_rel_dir.as_deref(),
            &workspace_deps,
            parsed.get("dev-dependencies"),
            EdgeKind::DevDependency,
        );
        collect_edge_section(
            tree,
            &mut edges,
            member,
            member_by_dir,
            member.workspace_root_rel_dir.as_deref(),
            &workspace_deps,
            parsed.get("build-dependencies"),
            EdgeKind::BuildDependency,
        );

        if let Some(target_table) = parsed.get("target").and_then(toml::Value::as_table) {
            for table in target_table.values().filter_map(toml::Value::as_table) {
                collect_edge_section(
                    tree,
                    &mut edges,
                    member,
                    member_by_dir,
                    member.workspace_root_rel_dir.as_deref(),
                    &workspace_deps,
                    table.get("dependencies"),
                    EdgeKind::TargetDependency,
                );
                collect_edge_section(
                    tree,
                    &mut edges,
                    member,
                    member_by_dir,
                    member.workspace_root_rel_dir.as_deref(),
                    &workspace_deps,
                    table.get("dev-dependencies"),
                    EdgeKind::TargetDevDependency,
                );
                collect_edge_section(
                    tree,
                    &mut edges,
                    member,
                    member_by_dir,
                    member.workspace_root_rel_dir.as_deref(),
                    &workspace_deps,
                    table.get("build-dependencies"),
                    EdgeKind::TargetBuildDependency,
                );
            }
        }
    }

    edges
}

fn collect_edge_section(
    tree: &ProjectTree,
    edges: &mut Vec<DependencyEdgeFacts>,
    member: &MemberDependencyFacts,
    member_by_dir: &BTreeMap<String, &MemberDependencyFacts>,
    workspace_root_rel_dir: Option<&str>,
    workspace_deps: &toml::map::Map<String, toml::Value>,
    section: Option<&toml::Value>,
    kind: EdgeKind,
) {
    let Some(table) = section.and_then(toml::Value::as_table) else {
        return;
    };

    for (alias, value) in table {
        let dep_table = value.as_table();
        let uses_workspace = dep_table
            .and_then(|table| table.get("workspace"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);
        let workspace_value = if uses_workspace {
            workspace_deps.get(alias)
        } else {
            None
        };
        let package_name = dependency_package_name(alias, dep_table, workspace_value);
        let direct_resolved_path_rel =
            extract_path(value, workspace_deps).map(|path| normalize_path(&member.rel_dir, &path));
        let inherited_resolved_path_rel = workspace_value
            .and_then(|workspace_value| extract_path(workspace_value, workspace_deps))
            .map(|path| normalize_path(workspace_root_rel_dir.unwrap_or(""), &path));
        let resolved_path_rel = direct_resolved_path_rel.or(inherited_resolved_path_rel);

        let resolved_member = resolved_path_rel
            .as_ref()
            .and_then(|resolved| member_by_dir.get(resolved).copied());
        let resolved_target_is_member = resolved_member.is_some();
        let resolved_target_exists = resolved_member.is_some()
            || resolved_path_rel.as_ref().is_some_and(|resolved| {
                tree.file_exists(&ProjectTree::join_rel(resolved, "Cargo.toml"))
            });
        let inferred_target_app_root = resolved_path_rel
            .as_ref()
            .and_then(|resolved| app_root_for_dir(resolved));
        let inferred_target_layer = resolved_path_rel.as_ref().and_then(|resolved| {
            let target_app_root = app_root_for_dir(resolved)?;
            if member.app_root_rel_dir.as_deref() == Some(target_app_root.as_str()) {
                layer_from_path(resolved)
            } else {
                None
            }
        });

        edges.push(DependencyEdgeFacts {
            source_name: member.name.clone(),
            source_rel_dir: member.rel_dir.clone(),
            source_cargo_rel_path: member.cargo_rel_path.clone(),
            source_layer: member.layer,
            source_app_root_rel_dir: member.app_root_rel_dir.clone(),
            dep_alias: alias.clone(),
            dep_package_name: package_name,
            kind,
            section_label: kind.label().to_owned(),
            resolved_target_rel_dir: resolved_member
                .map(|target| target.rel_dir.clone())
                .or(resolved_path_rel.clone()),
            resolved_target_exists,
            resolved_target_is_member,
            target_layer: resolved_member
                .and_then(|target| target.layer)
                .or(inferred_target_layer),
            target_app_root_rel_dir: resolved_member
                .and_then(|target| target.app_root_rel_dir.clone())
                .or(inferred_target_app_root),
            is_workspace_inherited: uses_workspace,
        });
    }
}

fn dependency_package_name(
    alias: &str,
    dep_table: Option<&toml::map::Map<String, toml::Value>>,
    workspace_value: Option<&toml::Value>,
) -> String {
    dep_table
        .and_then(|table| table.get("package"))
        .and_then(toml::Value::as_str)
        .or_else(|| {
            workspace_value
                .and_then(toml::Value::as_table)
                .and_then(|table| table.get("package"))
                .and_then(toml::Value::as_str)
        })
        .unwrap_or(alias)
        .to_owned()
}

fn extract_path(
    value: &toml::Value,
    workspace_deps: &toml::map::Map<String, toml::Value>,
) -> Option<String> {
    if let Some(path) = value
        .as_table()
        .and_then(|table| table.get("path"))
        .and_then(toml::Value::as_str)
    {
        return Some(path.to_owned());
    }

    value
        .as_table()
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        .filter(|workspace| *workspace)
        .and_then(|_| {
            let _ = workspace_deps;
            None
        })
}
