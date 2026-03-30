mod boundary;
mod cycles;
mod edges;
mod members;
mod workspaces;

use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsHexarchRoute;
use guardrail3_domain_config::types::CrateConfig;
use guardrail3_domain_project_tree::ProjectTree;

use self::boundary::{collect_boundary_configs, parse_guardrail_config};
use self::cycles::collect_same_layer_cycles;
use self::edges::collect_edges;
use self::members::{collect_members, dir_is_within_owned_hex_scope};
use self::workspaces::{best_workspace_for_member, discover_workspaces};

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
    pub cargo_parse_error: Option<String>,
    pub workspace_root_rel_dir: Option<String>,
    pub app_root_rel_dir: Option<String>,
    pub layer: Option<Layer>,
    pub allowed_deps: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct MemberManifestFailureFacts {
    pub name: String,
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub app_root_rel_dir: Option<String>,
    pub layer: Option<Layer>,
    pub parse_error: String,
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
    pub member_manifest_failures: Vec<MemberManifestFailureFacts>,
    pub edges: Vec<DependencyEdgeFacts>,
    pub boundary_configs: Vec<BoundaryConfigFacts>,
    pub cycles: Vec<CycleFacts>,
}

pub fn collect(tree: &ProjectTree, route: &RsHexarchRoute) -> DependencyFamilyFacts {
    let owned_app_roots = owned_app_roots(route);
    let guardrail_config = parse_guardrail_config(tree, route.guardrail_config_rel_path.as_deref());
    let workspaces = discover_workspaces(
        tree,
        &owned_app_roots,
        route.repo_root_cargo_rel_path.is_some(),
    );
    let workspace_for_member = best_workspace_for_member(&workspaces);
    let (members, member_manifest_failures) = collect_members(
        tree,
        &workspaces,
        &workspace_for_member,
        &owned_app_roots,
        guardrail_config.parsed.as_ref(),
    );
    let member_by_dir = members
        .iter()
        .map(|member| (member.rel_dir.clone(), member))
        .collect::<BTreeMap<_, _>>();
    let edges = collect_edges(tree, &workspaces, &members, &member_by_dir);
    let cycles = collect_same_layer_cycles(&edges, &member_by_dir);
    let boundary_configs = collect_boundary_configs(&owned_app_roots, &guardrail_config);

    DependencyFamilyFacts {
        workspaces,
        members,
        member_manifest_failures,
        edges,
        boundary_configs,
        cycles,
    }
}

#[cfg(test)]
pub(crate) fn collect_for_test_tree(tree: &ProjectTree) -> DependencyFamilyFacts {
    let route = crate::family_route_for_tests(tree);
    collect(tree, &route)
}

fn owned_app_roots(route: &RsHexarchRoute) -> BTreeSet<String> {
    route
        .roots
        .iter()
        .filter_map(|root| {
            let app_name = root.rel_dir.strip_prefix("apps/")?;
            if app_name.contains('/') {
                return None;
            }
            Some(root.rel_dir.clone())
        })
        .collect()
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
    app_config_names: BTreeSet<String>,
    raw_parse_succeeded: bool,
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
mod dependency_facts_tests;
