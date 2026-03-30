use std::collections::{BTreeMap, BTreeSet};

#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
use guardrail3_app_rs_family_mapper::RsClippyRoute;
use guardrail3_domain_project_tree::ProjectTree;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

#[cfg(test)]
use super::inputs::ConfigClippyInput;

mod cargo;
mod configs;
mod policy;

use self::cargo::collect_cargo_roots;
use self::configs::{
    collect_cargo_config_overrides, collect_configs, config_precedence, push_coverage_facts,
};
use self::policy::read_policy_map;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyRootKind {
    WorkspaceRoot,
    StandalonePackageRoot,
}

impl PolicyRootKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceRoot => "workspace root",
            Self::StandalonePackageRoot => "standalone package root",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClippyConfigFacts {
    pub(crate) rel_dir: String,
    pub(crate) rel_path: String,
    pub(crate) parsed: Option<toml::Value>,
    pub(crate) parse_error: Option<String>,
    pub(crate) policy_context_parse_error: Option<String>,
    pub(crate) profile_name: Option<String>,
    pub(crate) garde_enabled: bool,
    pub(crate) published_library_policy: bool,
}

#[derive(Debug, Clone)]
pub enum ForbiddenConfigReason {
    NotAllowedRoot,
    UnparseableCargoRoot {
        cargo_rel_path: String,
        parse_error: String,
    },
    ShadowedSameRoot {
        preferred_rel_path: String,
    },
}

#[derive(Debug, Clone)]
pub struct ForbiddenConfigFacts {
    pub(crate) config: ClippyConfigFacts,
    pub(crate) reason: ForbiddenConfigReason,
}

#[derive(Debug, Clone)]
pub struct CoveredRustUnitFacts {
    pub(crate) rel_dir: String,
    pub(crate) kind: PolicyRootKind,
    pub(crate) covering_config_rel: String,
}

#[derive(Debug, Clone)]
pub struct UncoveredRustUnitFacts {
    pub(crate) rel_dir: String,
    pub(crate) kind: PolicyRootKind,
}

#[derive(Debug, Clone)]
pub struct ClippyFacts {
    pub(crate) policy_context_parse_error: Option<String>,
    pub(crate) allowed_configs: Vec<ClippyConfigFacts>,
    pub(crate) forbidden_configs: Vec<ForbiddenConfigFacts>,
    pub(crate) cargo_config_overrides: Vec<CargoConfigOverrideFacts>,
    pub(crate) cargo_root_failures: Vec<CargoRootFailureFacts>,
    pub(crate) covered_units: Vec<CoveredRustUnitFacts>,
    pub(crate) uncovered_units: Vec<UncoveredRustUnitFacts>,
}

#[derive(Debug, Clone)]
pub struct CargoConfigOverrideFacts {
    pub(crate) rel_path: String,
    pub(crate) parse_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CargoRootFailureFacts {
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) parse_error: String,
}

#[derive(Debug, Clone)]
struct CargoRootFacts {
    rel_dir: String,
    cargo_rel_path: String,
    parse_error: Option<String>,
    has_workspace: bool,
    has_package: bool,
    workspace_members: Vec<String>,
}

#[derive(Debug, Clone)]
struct PolicySettings {
    profile_name: Option<String>,
    garde_enabled: bool,
}

#[derive(Debug, Clone)]
struct GuardrailPolicyFacts {
    parsed: Option<toml::Value>,
    parse_error: Option<String>,
    default_profile: Option<String>,
    default_garde: bool,
}

#[derive(Debug, Clone)]
struct ResolvedPolicyMap {
    map: BTreeMap<String, PolicySettings>,
    parse_error: Option<String>,
}

pub fn collect(tree: &ProjectTree, route: &RsClippyRoute) -> ClippyFacts {
    let cargo_roots = collect_cargo_roots(tree, route);
    let validation_scope = route.validation_scope();
    let routed_root_rels = route
        .roots()
        .iter()
        .map(|root| root.rel_dir().to_owned())
        .collect::<BTreeSet<_>>();
    let workspace_roots: BTreeSet<_> = cargo_roots
        .values()
        .filter(|facts| facts.parse_error.is_none())
        .filter(|facts| facts.has_workspace)
        .map(|facts| facts.rel_dir.clone())
        .collect();
    let workspace_members: BTreeSet<_> = cargo_roots
        .values()
        .filter(|facts| facts.parse_error.is_none())
        .flat_map(|facts| facts.workspace_members.iter().cloned())
        .collect();
    let standalone_package_roots: BTreeSet<_> = cargo_roots
        .values()
        .filter(|facts| facts.parse_error.is_none())
        .filter(|facts| facts.has_package && !workspace_members.contains(&facts.rel_dir))
        .map(|facts| facts.rel_dir.clone())
        .collect();
    let policy_map = read_policy_map(tree, &cargo_roots, &standalone_package_roots);
    let mut cargo_config_overrides =
        collect_cargo_config_overrides(tree, &routed_root_rels, &cargo_roots, validation_scope);

    let mut allowed_policy_roots = BTreeSet::new();
    let _ = allowed_policy_roots.insert(String::new());
    allowed_policy_roots.extend(workspace_roots.iter().cloned());
    allowed_policy_roots.extend(standalone_package_roots.iter().cloned());

    let configs = collect_configs(
        tree,
        &cargo_roots,
        &policy_map,
        &routed_root_rels,
        validation_scope,
    );
    let mut allowed_configs = Vec::new();
    let mut forbidden_configs = Vec::new();
    for config in configs {
        if let Some(cargo_root) = cargo_roots
            .get(&config.rel_dir)
            .filter(|facts| !facts.rel_dir.is_empty())
            .filter(|facts| facts.parse_error.is_some())
        {
            forbidden_configs.push(ForbiddenConfigFacts {
                config,
                reason: ForbiddenConfigReason::UnparseableCargoRoot {
                    cargo_rel_path: cargo_root.cargo_rel_path.clone(),
                    parse_error: cargo_root
                        .parse_error
                        .clone()
                        .expect("cargo root parse error"),
                },
            });
            continue;
        }
        if allowed_policy_roots.contains(&config.rel_dir) {
            allowed_configs.push(config);
        } else {
            forbidden_configs.push(ForbiddenConfigFacts {
                config,
                reason: ForbiddenConfigReason::NotAllowedRoot,
            });
        }
    }

    let mut deduped_allowed = Vec::new();
    let mut configs_by_dir = BTreeMap::<String, Vec<ClippyConfigFacts>>::new();
    for config in allowed_configs {
        configs_by_dir
            .entry(config.rel_dir.clone())
            .or_default()
            .push(config);
    }

    for (_rel_dir, mut same_root_configs) in configs_by_dir {
        same_root_configs.sort_by_key(|config| config_precedence(&config.rel_path));
        let mut same_root_iter = same_root_configs.into_iter();
        if let Some(preferred) = same_root_iter.next() {
            let preferred_rel_path = preferred.rel_path.clone();
            deduped_allowed.push(preferred);
            for config in same_root_iter {
                forbidden_configs.push(ForbiddenConfigFacts {
                    config,
                    reason: ForbiddenConfigReason::ShadowedSameRoot {
                        preferred_rel_path: preferred_rel_path.clone(),
                    },
                });
            }
        }
    }
    let mut allowed_configs = deduped_allowed;
    let mut cargo_root_failures = cargo_roots
        .values()
        .filter_map(|facts| {
            facts
                .parse_error
                .as_ref()
                .map(|parse_error: &String| CargoRootFailureFacts {
                    rel_dir: facts.rel_dir.clone(),
                    cargo_rel_path: facts.cargo_rel_path.clone(),
                    parse_error: parse_error.clone(),
                })
        })
        .collect::<Vec<_>>();

    let mut covered_units = Vec::new();
    let mut uncovered_units = Vec::new();
    for rel_dir in workspace_roots {
        push_coverage_facts(
            &rel_dir,
            PolicyRootKind::WorkspaceRoot,
            &allowed_configs,
            &mut covered_units,
            &mut uncovered_units,
        );
    }
    for rel_dir in standalone_package_roots {
        push_coverage_facts(
            &rel_dir,
            PolicyRootKind::StandalonePackageRoot,
            &allowed_configs,
            &mut covered_units,
            &mut uncovered_units,
        );
    }

    covered_units.sort_by(|a, b| a.rel_dir.cmp(&b.rel_dir));
    uncovered_units.sort_by(|a, b| a.rel_dir.cmp(&b.rel_dir));
    allowed_configs.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    forbidden_configs.sort_by(|a, b| a.config.rel_path.cmp(&b.config.rel_path));
    cargo_config_overrides.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    cargo_root_failures.sort_by(|a, b| a.cargo_rel_path.cmp(&b.cargo_rel_path));

    ClippyFacts {
        policy_context_parse_error: policy_map.parse_error,
        allowed_configs,
        forbidden_configs,
        cargo_config_overrides,
        cargo_root_failures,
        covered_units,
        uncovered_units,
    }
}

#[cfg(test)]
pub(crate) fn collect_for_tests(tree: &ProjectTree) -> ClippyFacts {
    collect(tree, &family_route_for_tests(tree, None))
}

#[cfg(test)]
pub(crate) fn collect_with_validation_scope_for_tests(
    tree: &ProjectTree,
    validation_scope: &str,
) -> ClippyFacts {
    collect(tree, &family_route_for_tests(tree, Some(validation_scope)))
}

#[cfg(test)]
pub(crate) fn config_input_for_tests<'a>(
    facts: &'a ClippyFacts,
    rel_path: &str,
) -> ConfigClippyInput<'a> {
    let config = facts
        .allowed_configs
        .iter()
        .find(|config| config.rel_path == rel_path)
        .expect("expected clippy config facts");
    ConfigClippyInput::new(config)
}

#[cfg(test)]
fn family_route_for_tests(tree: &ProjectTree, validation_scope: Option<&str>) -> RsClippyRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let selected = RustFamilySelection::new(std::collections::BTreeSet::from([
        RustValidateFamily::Clippy,
    ]));
    FamilyMapper::new(tree, &scope, None, &selected, None)
        .with_validation_scope(validation_scope)
        .map_rs_clippy()
}

#[cfg(test)]
#[path = "facts_tests/mod.rs"] // reason: test-only sidecar module wiring
mod facts_tests;
