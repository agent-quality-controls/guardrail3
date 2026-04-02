mod boundary;
mod cycles;
mod edges;
mod members;
mod workspaces;

use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsHexarchRoute;
use guardrail3_domain_config::types::{CrateConfig, EscapeHatchConfig};
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

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
    pub(crate) root_rel_dir: String,
    pub(crate) member_dirs: Vec<String>,
    pub(crate) workspace_dependencies: toml::map::Map<String, toml::Value>,
    pub(crate) patch_entries: Vec<PatchEntryFacts>,
}

#[derive(Debug, Clone)]
pub struct PatchEntryFacts {
    pub(crate) cargo_rel_path: String,
    pub(crate) key: String,
    pub(crate) resolved_rel_dir: String,
    pub(crate) target_layer: Option<Layer>,
    pub(crate) escape_hatch_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MemberDependencyFacts {
    pub(crate) name: String,
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) cargo_parse_error: Option<String>,
    pub(crate) workspace_root_rel_dir: Option<String>,
    pub(crate) app_root_rel_dir: Option<String>,
    pub(crate) layer: Option<Layer>,
    pub(crate) allowed_deps: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct MemberManifestFailureFacts {
    pub(crate) name: String,
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) parse_error: String,
}

#[derive(Debug, Clone)]
pub struct DependencyEdgeFacts {
    pub(crate) source_name: String,
    pub(crate) source_rel_dir: String,
    pub(crate) source_cargo_rel_path: String,
    pub(crate) source_layer: Option<Layer>,
    pub(crate) source_app_root_rel_dir: Option<String>,
    pub(crate) dep_alias: String,
    pub(crate) dep_package_name: String,
    pub(crate) kind: EdgeKind,
    pub(crate) section_label: String,
    pub(crate) resolved_target_rel_dir: Option<String>,
    pub(crate) resolved_target_exists: bool,
    pub(crate) resolved_target_is_member: bool,
    pub(crate) target_layer: Option<Layer>,
    pub(crate) target_app_root_rel_dir: Option<String>,
    pub(crate) is_workspace_inherited: bool,
}

#[derive(Debug, Clone)]
pub struct BoundaryConfigFacts {
    pub(crate) rel_dir: String,
    pub(crate) has_config_entry: bool,
    pub(crate) is_app_boundary: bool,
    pub(crate) parse_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CycleFacts {
    pub(crate) layer: Layer,
    pub(crate) members: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DependencyFamilyFacts {
    pub(crate) workspaces: Vec<WorkspaceFacts>,
    pub(crate) members: Vec<MemberDependencyFacts>,
    pub(crate) member_manifest_failures: Vec<MemberManifestFailureFacts>,
    pub(crate) edges: Vec<DependencyEdgeFacts>,
    pub(crate) boundary_configs: Vec<BoundaryConfigFacts>,
    pub(crate) cycles: Vec<CycleFacts>,
}

pub fn collect(tree: &ProjectTree, route: &RsHexarchRoute) -> DependencyFamilyFacts {
    let owned_app_roots = owned_app_roots(route);
    let guardrail_config = parse_guardrail_config(tree, route.guardrail_config_rel_path());
    let mut workspaces = discover_workspaces(
        tree,
        &owned_app_roots,
        route.repo_root_cargo_rel_path().is_some(),
    );
    if let Some(parsed_guardrail) = guardrail_config.parsed.as_ref() {
        for workspace in &mut workspaces {
            for patch in &mut workspace.patch_entries {
                let selector = format!("{}@{}", patch.key, patch.resolved_rel_dir);
                patch.escape_hatch_reason = parsed_guardrail
                    .escape_hatches
                    .iter()
                    .find(|entry| {
                        entry.family() == "hexarch"
                            && entry.file() == patch.cargo_rel_path
                            && entry.kind() == "patch_replace"
                            && entry.selector() == selector
                    })
                    .map(|entry| entry.reason().to_owned());
            }
        }
    }
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
        .roots()
        .iter()
        .filter_map(|root| {
            let app_name = root.rel_dir().strip_prefix("apps/")?;
            if app_name.contains('/') {
                return None;
            }
            Some(root.rel_dir().to_owned())
        })
        .collect()
}

#[derive(Debug, Clone)]
struct ParsedGuardrailConfig {
    root_profile_name: Option<String>,
    app_configs: BTreeMap<String, CrateConfig>,
    packages_config: Option<CrateConfig>,
    escape_hatches: Vec<EscapeHatchConfig>,
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
mod tests;
