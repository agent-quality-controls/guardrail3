use std::collections::{BTreeMap, BTreeSet};

use crate::facts_support;
use guardrail3_app_rs_family_mapper::RsDenyRoute;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_domain_project_tree::ProjectTree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyRootKind {
    WorkspaceRoot,
}

impl PolicyRootKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceRoot => "workspace root",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DenyConfigFacts {
    pub(crate) policy_root_rel: String,
    pub(crate) rel_path: String,
    pub(crate) file_kind: String,
    pub(crate) parsed: Option<toml::Value>,
    pub(crate) parse_error: Option<String>,
    pub(crate) profile_name: Option<String>,
    pub(crate) policy_context_valid: bool,
}

#[derive(Debug, Clone)]
pub struct ForbiddenDenyConfigFacts {
    pub(crate) policy_root_rel: String,
    pub(crate) rel_path: String,
    pub(crate) file_kind: String,
    pub(crate) parse_error: Option<String>,
    pub(crate) shadowed_root_rel: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CoveredRustUnitFacts {
    pub(crate) rel_dir: String,
    pub(crate) kind: PolicyRootKind,
    pub(crate) covering_config_rel: String,
    pub(crate) quiet_if_self_hosted: bool,
}

#[derive(Debug, Clone)]
pub struct UncoveredRustUnitFacts {
    pub(crate) rel_dir: String,
    pub(crate) kind: PolicyRootKind,
}

#[derive(Debug, Clone)]
pub struct DenyFacts {
    pub(crate) policy_context_parse_error: Option<String>,
    pub(crate) linted_configs: Vec<DenyConfigFacts>,
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) allowed_configs: Vec<DenyConfigFacts>,
    pub(crate) forbidden_configs: Vec<ForbiddenDenyConfigFacts>,
    pub(crate) same_root_conflicts: Vec<SameRootConflictFacts>,
    pub(crate) covered_units: Vec<CoveredRustUnitFacts>,
    pub(crate) uncovered_units: Vec<UncoveredRustUnitFacts>,
}

#[derive(Debug, Clone)]
pub struct SameRootConflictFacts {
    pub(crate) policy_root_rel: String,
    pub(crate) rel_paths: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct CargoRootFacts {
    pub(crate) rel_dir: String,
    pub(crate) has_workspace: bool,
    pub(crate) workspace_members: Vec<String>,
}

pub fn collect(tree: &ProjectTree, route: &RsDenyRoute) -> DenyFacts {
    let cargo_roots = collect_cargo_roots(tree, route);
    let routed_root_rels = route
        .roots()
        .iter()
        .map(|root| root.rel_dir().to_owned())
        .collect::<BTreeSet<_>>();
    let workspace_roots: BTreeSet<_> = cargo_roots
        .iter()
        .filter(|(rel_dir, _facts)| routed_root_rels.contains(*rel_dir))
        .filter(|(_rel_dir, facts)| facts.has_workspace)
        .map(|(rel_dir, _facts)| rel_dir.clone())
        .collect();
    let mut allowed_policy_roots = BTreeSet::new();
    allowed_policy_roots.extend(workspace_roots.iter().cloned());
    let profile_map_facts = facts_support::read_profile_map(tree, &cargo_roots);
    let policy_context_valid = profile_map_facts.parse_error.is_none();

    let configs = collect_configs(
        tree,
        route,
        &profile_map_facts.map,
        &routed_root_rels,
        route.validation_scope(),
        policy_context_valid,
    );
    let mut linted_configs = Vec::new();
    let mut allowed_configs = Vec::new();
    let mut forbidden_configs = Vec::new();
    for config in configs {
        linted_configs.push(config.clone());
        if allowed_policy_roots.contains(&config.policy_root_rel) {
            allowed_configs.push(config);
        } else {
            let shadowed_root_rel = facts_support::nearest_allowed_ancestor(
                &config.policy_root_rel,
                &allowed_policy_roots,
            );
            forbidden_configs.push(ForbiddenDenyConfigFacts {
                policy_root_rel: config.policy_root_rel,
                rel_path: config.rel_path,
                file_kind: config.file_kind,
                parse_error: config.parse_error,
                shadowed_root_rel,
            });
        }
    }
    let mut same_root_conflicts = collect_same_root_conflicts(&allowed_configs);

    let mut covered_units = Vec::new();
    let mut uncovered_units = Vec::new();
    for rel_dir in workspace_roots {
        facts_support::push_coverage_facts(
            tree,
            &rel_dir,
            PolicyRootKind::WorkspaceRoot,
            &allowed_configs,
            &mut covered_units,
            &mut uncovered_units,
        );
    }

    linted_configs.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    allowed_configs.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    forbidden_configs.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    same_root_conflicts.sort_by(|a, b| a.policy_root_rel.cmp(&b.policy_root_rel));
    covered_units.sort_by(|a, b| a.rel_dir.cmp(&b.rel_dir));
    uncovered_units.sort_by(|a, b| a.rel_dir.cmp(&b.rel_dir));

    DenyFacts {
        policy_context_parse_error: profile_map_facts.parse_error,
        linted_configs,
        allowed_configs,
        forbidden_configs,
        same_root_conflicts,
        covered_units,
        uncovered_units,
    }
}

fn collect_cargo_roots(
    tree: &ProjectTree,
    route: &RsDenyRoute,
) -> BTreeMap<String, CargoRootFacts> {
    let routed_root_rels = route
        .roots()
        .iter()
        .map(|root| root.rel_dir())
        .collect::<BTreeSet<_>>();

    route
        .family_files()
        .iter()
        .filter(|file| file.kind() == RustFamilyFileKind::CargoToml && file.exact_rust_root_owner())
        .filter(|file| {
            routed_root_rels.contains(file.logical_owner_rel())
                || route.family_files().iter().any(|candidate| {
                    matches!(
                        candidate.kind(),
                        RustFamilyFileKind::DenyToml
                            | RustFamilyFileKind::DenyDotToml
                            | RustFamilyFileKind::CargoDenyToml
                    ) && candidate.logical_owner_rel() == file.logical_owner_rel()
                })
        })
        .map(|file| file.logical_owner_rel().to_owned())
        .map(|rel_dir| {
            let rel_path = if rel_dir.is_empty() {
                "Cargo.toml".to_owned()
            } else {
                ProjectTree::join_rel(&rel_dir, "Cargo.toml")
            };
            let parsed = tree
                .file_content(&rel_path)
                .and_then(|content| toml::from_str::<toml::Value>(content).ok());
            let facts = parsed.as_ref().map_or_else(
                || CargoRootFacts {
                    rel_dir: rel_dir.clone(),
                    has_workspace: false,
                    workspace_members: Vec::new(),
                },
                |parsed| CargoRootFacts {
                    rel_dir: rel_dir.clone(),
                    has_workspace: parsed.get("workspace").is_some(),
                    workspace_members: parse_workspace_members(tree, &rel_dir, parsed),
                },
            );
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

fn collect_configs(
    tree: &ProjectTree,
    route: &RsDenyRoute,
    profile_map: &BTreeMap<String, Option<String>>,
    _routed_root_rels: &BTreeSet<String>,
    validation_scope: Option<&str>,
    policy_context_valid: bool,
) -> Vec<DenyConfigFacts> {
    let mut configs = Vec::new();
    let mut seen_paths = BTreeSet::new();
    for file in route.family_files() {
        let (policy_root_rel, file_kind) = match file.kind() {
            RustFamilyFileKind::DenyToml => (file.logical_owner_rel(), "deny.toml"),
            RustFamilyFileKind::DenyDotToml => (file.logical_owner_rel(), ".deny.toml"),
            RustFamilyFileKind::CargoDenyToml => (file.logical_owner_rel(), ".cargo/deny.toml"),
            _ => continue,
        };
        if !overlaps_validation_scope(policy_root_rel, validation_scope) {
            continue;
        }
        push_config_if_present(
            tree,
            policy_root_rel,
            file.rel_path(),
            file_kind,
            profile_map,
            policy_context_valid,
            &mut seen_paths,
            &mut configs,
        );
    }

    configs
}

fn collect_same_root_conflicts(allowed_configs: &[DenyConfigFacts]) -> Vec<SameRootConflictFacts> {
    let mut grouped = BTreeMap::<String, Vec<String>>::new();

    for config in allowed_configs {
        grouped
            .entry(config.policy_root_rel.clone())
            .or_default()
            .push(config.rel_path.clone());
    }

    grouped
        .into_iter()
        .filter_map(|(policy_root_rel, mut rel_paths)| {
            rel_paths.sort_by_key(|rel_path| deny_config_precedence(rel_path));
            rel_paths.dedup();
            (rel_paths.len() > 1).then_some(SameRootConflictFacts {
                policy_root_rel,
                rel_paths,
            })
        })
        .collect()
}

fn overlaps_validation_scope(rel_path: &str, validation_scope: Option<&str>) -> bool {
    validation_scope.is_none_or(|scope_rel| {
        path_is_under(rel_path, scope_rel) || path_is_under(scope_rel, rel_path)
    })
}

fn deny_config_precedence(rel_path: &str) -> usize {
    if rel_path.ends_with("/.cargo/deny.toml") || rel_path == ".cargo/deny.toml" {
        0
    } else if rel_path.ends_with("/.deny.toml") || rel_path == ".deny.toml" {
        1
    } else {
        2
    }
}

fn path_is_under(rel_path: &str, parent_rel: &str) -> bool {
    parent_rel.is_empty()
        || rel_path == parent_rel
        || rel_path
            .strip_prefix(parent_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}

fn push_config_if_present(
    tree: &ProjectTree,
    policy_root_rel: &str,
    rel_path: &str,
    file_kind: &str,
    profile_map: &BTreeMap<String, Option<String>>,
    policy_context_valid: bool,
    seen_paths: &mut BTreeSet<String>,
    configs: &mut Vec<DenyConfigFacts>,
) {
    if !seen_paths.insert(rel_path.to_owned()) {
        return;
    }
    let Some(content) = tree.file_content(rel_path) else {
        return;
    };
    let profile_name = facts_support::profile_for(policy_root_rel, profile_map);
    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => configs.push(DenyConfigFacts {
            policy_root_rel: policy_root_rel.to_owned(),
            rel_path: rel_path.to_owned(),
            file_kind: file_kind.to_owned(),
            parsed: Some(parsed),
            parse_error: None,
            profile_name,
            policy_context_valid,
        }),
        Err(err) => configs.push(DenyConfigFacts {
            policy_root_rel: policy_root_rel.to_owned(),
            rel_path: rel_path.to_owned(),
            file_kind: file_kind.to_owned(),
            parsed: None,
            parse_error: Some(err.to_string()),
            profile_name,
            policy_context_valid,
        }),
    }
}

#[cfg(test)]
pub(crate) fn collect_for_test(tree: &ProjectTree) -> DenyFacts {
    crate::collect_facts_for_test(tree)
}

#[cfg(test)]
pub(crate) use ::test_support::{build_fixture_deny_toml, dir_entry, project_tree};
#[cfg(test)]
#[path = "facts_tests/mod.rs"] // reason: test-only sidecar module wiring
mod facts_tests;
