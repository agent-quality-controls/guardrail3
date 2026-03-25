use std::collections::{BTreeMap, BTreeSet};

use guardrail3_domain_config::types::{CrateConfig, GuardrailConfig};
use guardrail3_domain_project_tree::ProjectTree;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Layer {
    Domain,
    Ports,
    App,
    Adapters,
}

impl Layer {
    #[must_use]
    pub const fn forbidden(self) -> &'static [Self] {
        match self {
            Self::Domain => &[Self::Ports, Self::App, Self::Adapters],
            Self::Ports => &[Self::App, Self::Adapters],
            Self::App => &[Self::Adapters],
            Self::Adapters => &[],
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Domain => "domain",
            Self::Ports => "ports",
            Self::App => "app",
            Self::Adapters => "adapters",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Dependency,
    DevDependency,
    BuildDependency,
    TargetDependency,
    TargetDevDependency,
    TargetBuildDependency,
}

impl EdgeKind {
    #[must_use]
    pub const fn is_dev(self) -> bool {
        matches!(self, Self::DevDependency | Self::TargetDevDependency)
    }

    #[must_use]
    pub const fn is_target(self) -> bool {
        matches!(
            self,
            Self::TargetDependency | Self::TargetDevDependency | Self::TargetBuildDependency
        )
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Dependency => "dependencies",
            Self::DevDependency => "dev-dependencies",
            Self::BuildDependency => "build-dependencies",
            Self::TargetDependency => "target.*.dependencies",
            Self::TargetDevDependency => "target.*.dev-dependencies",
            Self::TargetBuildDependency => "target.*.build-dependencies",
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkspaceFacts {
    pub root_rel_dir: String,
    pub member_dirs: Vec<String>,
    pub workspace_dependencies: toml::map::Map<String, toml::Value>,
    pub patch_entries: Vec<PatchEntryFacts>,
}

#[derive(Debug, Clone)]
pub struct PatchEntryFacts {
    pub cargo_rel_path: String,
    pub key: String,
    pub resolved_rel_dir: String,
    pub target_layer: Option<Layer>,
}

#[derive(Debug, Clone)]
pub struct MemberDependencyFacts {
    pub name: String,
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub workspace_root_rel_dir: Option<String>,
    pub app_root_rel_dir: Option<String>,
    pub layer: Option<Layer>,
    pub allowed_deps: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct DependencyEdgeFacts {
    pub source_name: String,
    pub source_rel_dir: String,
    pub source_cargo_rel_path: String,
    pub source_layer: Option<Layer>,
    pub source_app_root_rel_dir: Option<String>,
    pub dep_alias: String,
    pub dep_package_name: String,
    pub kind: EdgeKind,
    pub section_label: String,
    pub resolved_target_rel_dir: Option<String>,
    pub resolved_target_exists: bool,
    pub resolved_target_is_member: bool,
    pub target_layer: Option<Layer>,
    pub target_app_root_rel_dir: Option<String>,
    pub is_workspace_inherited: bool,
}

#[derive(Debug, Clone)]
pub struct BoundaryConfigFacts {
    pub rel_dir: String,
    pub has_config_entry: bool,
    pub is_app_boundary: bool,
    pub parse_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CycleFacts {
    pub layer: Layer,
    pub members: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DependencyFamilyFacts {
    pub workspaces: Vec<WorkspaceFacts>,
    pub members: Vec<MemberDependencyFacts>,
    pub edges: Vec<DependencyEdgeFacts>,
    pub boundary_configs: Vec<BoundaryConfigFacts>,
    pub cycles: Vec<CycleFacts>,
}

pub fn collect(tree: &ProjectTree) -> DependencyFamilyFacts {
    let guardrail_config = parse_guardrail_config(tree);
    let workspaces = discover_workspaces(tree);
    let workspace_for_member = best_workspace_for_member(&workspaces);
    let members = collect_members(
        tree,
        &workspaces,
        &workspace_for_member,
        guardrail_config.parsed.as_ref(),
    );
    let member_by_dir = members
        .iter()
        .map(|member| (member.rel_dir.clone(), member))
        .collect::<BTreeMap<_, _>>();
    let edges = collect_edges(tree, &workspaces, &members, &member_by_dir);
    let cycles = collect_same_layer_cycles(&edges, &member_by_dir);
    let boundary_configs = collect_boundary_configs(&members, &guardrail_config);

    DependencyFamilyFacts {
        workspaces,
        members,
        edges,
        boundary_configs,
        cycles,
    }
}

fn collect_boundary_configs(
    members: &[MemberDependencyFacts],
    guardrail: &GuardrailConfigSnapshot,
) -> Vec<BoundaryConfigFacts> {
    if let Some(parse_error) = &guardrail.parse_error {
        return vec![BoundaryConfigFacts {
            rel_dir: "guardrail3.toml".to_owned(),
            has_config_entry: false,
            is_app_boundary: false,
            parse_error: Some(parse_error.clone()),
        }];
    }

    let mut boundaries = BTreeMap::<String, BoundaryConfigFacts>::new();
    for member in members {
        if let Some(app_root) = &member.app_root_rel_dir {
            let _ = boundaries.entry(app_root.clone()).or_insert_with(|| {
                let app_name = app_root.rsplit('/').next().unwrap_or(app_root);
                let has_config_entry = guardrail
                    .parsed
                    .as_ref()
                    .is_some_and(|guardrail| guardrail.app_configs.contains_key(app_name));
                BoundaryConfigFacts {
                    rel_dir: app_root.clone(),
                    has_config_entry,
                    is_app_boundary: true,
                    parse_error: None,
                }
            });
        }
    }
    boundaries.into_values().collect()
}

#[derive(Debug, Clone)]
struct ParsedGuardrailConfig {
    root_profile_name: Option<String>,
    app_configs: BTreeMap<String, CrateConfig>,
    packages_config: Option<CrateConfig>,
}

#[derive(Debug, Clone, Default)]
struct GuardrailConfigSnapshot {
    parsed: Option<ParsedGuardrailConfig>,
    parse_error: Option<String>,
}

fn parse_guardrail_config(tree: &ProjectTree) -> GuardrailConfigSnapshot {
    let Some(content) = tree.file_content("guardrail3.toml") else {
        return GuardrailConfigSnapshot::default();
    };
    match toml::from_str::<GuardrailConfig>(content) {
        Ok(config) => GuardrailConfigSnapshot {
            parsed: Some(ParsedGuardrailConfig {
                root_profile_name: config.profile.map(|profile| profile.name),
                app_configs: config
                    .rust
                    .as_ref()
                    .and_then(|rust| rust.apps.clone())
                    .unwrap_or_default(),
                packages_config: config.rust.and_then(|rust| rust.packages),
            }),
            parse_error: None,
        },
        Err(parse_error) => GuardrailConfigSnapshot {
            parsed: None,
            parse_error: Some(parse_error.to_string()),
        },
    }
}

fn discover_workspaces(tree: &ProjectTree) -> Vec<WorkspaceFacts> {
    let mut workspaces = Vec::new();
    let mut seen = BTreeSet::new();

    for dir in std::iter::once(String::new()).chain(tree.dirs_with_file("Cargo.toml")) {
        if !seen.insert(dir.clone()) {
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
                if let Some(path_value) = extract_path(value, &toml::map::Map::new()) {
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
                    });
                }
            }
        }
    }

    if let Some(replace_table) = parsed.get("replace").and_then(toml::Value::as_table) {
        for (key, value) in replace_table {
            if let Some(path_value) = extract_path(value, &toml::map::Map::new()) {
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
                });
            }
        }
    }

    patches
}

fn best_workspace_for_member(workspaces: &[WorkspaceFacts]) -> BTreeMap<String, String> {
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

fn collect_members(
    tree: &ProjectTree,
    workspaces: &[WorkspaceFacts],
    workspace_for_member: &BTreeMap<String, String>,
    guardrail: Option<&ParsedGuardrailConfig>,
) -> Vec<MemberDependencyFacts> {
    let mut member_dirs = BTreeSet::new();
    for workspace in workspaces {
        member_dirs.extend(workspace.member_dirs.iter().cloned());
    }

    let mut members = Vec::new();
    for rel_dir in member_dirs {
        let cargo_rel_path = format!("{rel_dir}/Cargo.toml");
        let cargo_content = tree.file_content(&cargo_rel_path);
        let name = match cargo_content {
            Some(content) => match toml::from_str::<toml::Value>(content) {
                Ok(parsed) => {
                    (
                        parsed
                            .get("package")
                            .and_then(|value| value.get("name"))
                            .and_then(toml::Value::as_str)
                            .map(str::to_owned)
                            .unwrap_or_else(|| {
                                rel_dir.rsplit('/').next().unwrap_or(&rel_dir).to_owned()
                            }),
                        None::<String>,
                    )
                        .0
                }
                Err(_) => rel_dir.rsplit('/').next().unwrap_or(&rel_dir).to_owned(),
            },
            None => rel_dir.rsplit('/').next().unwrap_or(&rel_dir).to_owned(),
        };

        let app_root_rel_dir = app_root_for_dir(&rel_dir);
        let (_profile_name, allowed_deps) =
            profile_and_allowed_deps_for_member(&rel_dir, guardrail);

        members.push(MemberDependencyFacts {
            name,
            rel_dir: rel_dir.clone(),
            cargo_rel_path,
            workspace_root_rel_dir: workspace_for_member.get(&rel_dir).cloned(),
            app_root_rel_dir,
            layer: layer_for_member(&rel_dir, guardrail),
            allowed_deps,
        });
    }

    members.sort_by(|left, right| left.rel_dir.cmp(&right.rel_dir));
    members
}

fn collect_edges(
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

fn collect_same_layer_cycles(
    edges: &[DependencyEdgeFacts],
    member_by_dir: &BTreeMap<String, &MemberDependencyFacts>,
) -> Vec<CycleFacts> {
    let mut graph: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for edge in edges
        .iter()
        .filter(|edge| !edge.kind.is_dev() && !edge.kind.is_target())
    {
        let Some(target_rel) = &edge.resolved_target_rel_dir else {
            continue;
        };
        if !member_by_dir.contains_key(target_rel) {
            continue;
        }
        graph
            .entry(edge.source_rel_dir.clone())
            .or_default()
            .push(target_rel.clone());
    }

    let mut cycles = Vec::new();
    let mut seen = BTreeSet::new();
    for start in graph.keys() {
        let mut stack = Vec::<String>::new();
        dfs_cycle(
            start,
            start,
            &graph,
            &mut stack,
            &mut seen,
            &mut cycles,
            member_by_dir,
        );
    }
    cycles
}

fn dfs_cycle(
    start: &str,
    node: &str,
    graph: &BTreeMap<String, Vec<String>>,
    stack: &mut Vec<String>,
    seen: &mut BTreeSet<String>,
    cycles: &mut Vec<CycleFacts>,
    member_by_dir: &BTreeMap<String, &MemberDependencyFacts>,
) {
    stack.push(node.to_owned());
    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if neighbor == start && stack.len() > 1 {
                let cycle = canonical_cycle(stack);
                let cycle_key = cycle.join(" -> ");
                if seen.insert(cycle_key) {
                    let member_layers = cycle
                        .iter()
                        .map(|member| member_by_dir.get(member).and_then(|facts| facts.layer))
                        .collect::<Vec<_>>();
                    if let Some(layer) = member_layers.first().copied().flatten().filter(|layer| {
                        member_layers
                            .iter()
                            .all(|candidate| *candidate == Some(*layer))
                    }) {
                        cycles.push(CycleFacts {
                            layer,
                            members: cycle,
                        });
                    }
                }
            } else if !stack.contains(neighbor) {
                dfs_cycle(start, neighbor, graph, stack, seen, cycles, member_by_dir);
            }
        }
    }
    let _ = stack.pop();
}

fn canonical_cycle(stack: &[String]) -> Vec<String> {
    let mut cycle = stack.to_vec();
    if cycle.is_empty() {
        return cycle;
    }
    let (min_index, _) = cycle
        .iter()
        .enumerate()
        .min_by(|(_, left), (_, right)| left.cmp(right))
        .expect("non-empty cycle");
    cycle.rotate_left(min_index);
    cycle
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

fn profile_and_allowed_deps_for_member(
    rel_dir: &str,
    guardrail: Option<&ParsedGuardrailConfig>,
) -> (Option<String>, BTreeSet<String>) {
    let Some(guardrail) = guardrail else {
        return (None, BTreeSet::new());
    };

    if let Some(app_root) = app_root_for_dir(rel_dir) {
        let app_name = app_root.rsplit('/').next().unwrap_or(&app_root);
        if let Some(config) = guardrail.app_configs.get(app_name) {
            return (
                config.profile.clone().or(config.type_.clone()),
                config
                    .allowed_deps
                    .clone()
                    .unwrap_or_default()
                    .into_iter()
                    .collect(),
            );
        }
    }

    if rel_dir.starts_with("packages/") {
        if let Some(config) = &guardrail.packages_config {
            return (
                config.profile.clone().or(config.type_.clone()),
                config
                    .allowed_deps
                    .clone()
                    .unwrap_or_default()
                    .into_iter()
                    .collect(),
            );
        }
    }

    (guardrail.root_profile_name.clone(), BTreeSet::new())
}

fn layer_for_member(rel_dir: &str, guardrail: Option<&ParsedGuardrailConfig>) -> Option<Layer> {
    layer_from_path(rel_dir).or_else(|| {
        if rel_dir.starts_with("packages/") {
            guardrail
                .and_then(|guardrail| guardrail.packages_config.as_ref())
                .and_then(|config| config.layer.as_deref())
                .and_then(layer_from_config)
        } else {
            None
        }
    })
}

#[must_use]
pub fn layer_from_config(value: &str) -> Option<Layer> {
    match value {
        "domain" | "pure" => Some(Layer::Domain),
        "ports" => Some(Layer::Ports),
        "app" => Some(Layer::App),
        "adapters" | "composition-root" => Some(Layer::Adapters),
        _ => None,
    }
}

#[must_use]
pub fn contains_segment(path: &str, segment: &str) -> bool {
    path.split('/').any(|part| part == segment)
}

#[must_use]
pub fn layer_from_path(path: &str) -> Option<Layer> {
    if contains_segment(path, "domain") {
        Some(Layer::Domain)
    } else if contains_segment(path, "ports") {
        Some(Layer::Ports)
    } else if contains_segment(path, "app") {
        Some(Layer::App)
    } else if contains_segment(path, "adapters") {
        Some(Layer::Adapters)
    } else {
        None
    }
}

#[must_use]
pub fn app_root_for_dir(rel_dir: &str) -> Option<String> {
    let mut parts = rel_dir.split('/');
    match (parts.next(), parts.next()) {
        (Some("apps"), Some(app)) => Some(format!("apps/{app}")),
        _ => None,
    }
}

#[must_use]
pub fn normalize_path(base: &str, rel: &str) -> String {
    let mut parts = base.split('/').collect::<Vec<_>>();
    for segment in rel.split('/') {
        match segment {
            ".." => {
                let _ = parts.pop();
            }
            "." | "" => {}
            value => parts.push(value),
        }
    }
    parts.join("/")
}

#[cfg(test)]
#[path = "dependency_facts_tests/mod.rs"]
mod tests;
