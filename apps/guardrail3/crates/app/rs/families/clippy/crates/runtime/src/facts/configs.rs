use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsClippyRoute;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;

use super::policy::{policy_settings_for, published_library_policy};
use super::{
    CargoConfigOverrideFacts, CargoRootFacts, ClippyConfigFacts, CoveredRustUnitFacts,
    PolicyRootKind, ResolvedPolicyMap, UncoveredRustUnitFacts,
};

pub(super) fn config_precedence(rel_path: &str) -> usize {
    if rel_path.ends_with(".clippy.toml") {
        return 0;
    }
    if rel_path.ends_with("clippy.toml") && !rel_path.ends_with(".clippy.toml") {
        return 1;
    }
    2
}

pub(super) fn collect_configs(
    tree: &ProjectTree,
    route: &RsClippyRoute,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    policy_map: &ResolvedPolicyMap,
    _routed_root_rels: &BTreeSet<String>,
    validation_scope: Option<&str>,
) -> Vec<ClippyConfigFacts> {
    let paths = route
        .family_files()
        .iter()
        .filter_map(|file| match file.kind() {
            RustFamilyFileKind::ClippyToml | RustFamilyFileKind::ClippyDotToml => Some((
                file.logical_owner_rel().to_owned(),
                file.rel_path().to_owned(),
            )),
            _ => None,
        })
        .filter(|(rel_dir, _)| config_dir_is_relevant(rel_dir, validation_scope))
        .collect::<Vec<_>>();

    paths
        .into_iter()
        .map(|(rel_dir, rel_path)| {
            let settings = policy_settings_for(rel_dir.as_str(), &policy_map.map);
            let published_library_policy = published_library_policy(
                tree,
                cargo_roots,
                rel_dir.as_str(),
                settings.profile_name.as_deref(),
            );
            parse_config(
                tree,
                &rel_dir,
                &rel_path,
                policy_map.parse_error.clone(),
                settings.profile_name,
                settings.garde_enabled,
                published_library_policy,
            )
        })
        .collect()
}

pub(super) fn collect_cargo_config_overrides(
    tree: &ProjectTree,
    route: &RsClippyRoute,
    _routed_root_rels: &BTreeSet<String>,
    _cargo_roots: &BTreeMap<String, CargoRootFacts>,
    validation_scope: Option<&str>,
) -> Vec<CargoConfigOverrideFacts> {
    let rel_paths = route
        .family_files()
        .iter()
        .filter_map(|file| match file.kind() {
            RustFamilyFileKind::CargoConfigToml | RustFamilyFileKind::CargoConfigLegacy => {
                Some((file.logical_owner_rel(), file.rel_path().to_owned()))
            }
            _ => None,
        })
        .filter(|(owner_rel, _)| cargo_config_owner_is_relevant(owner_rel, validation_scope))
        .map(|(_, rel_path)| rel_path)
        .collect::<Vec<_>>();

    rel_paths
        .into_iter()
        .filter_map(|rel_path| {
            let parsed = match tree.file_content(&rel_path) {
                Some(content) => toml::from_str::<toml::Value>(content)
                    .map(Some)
                    .map_err(|err| err.to_string()),
                None => Err("cargo config content missing from ProjectTree".to_owned()),
            };
            match parsed {
                Ok(Some(parsed)) => {
                    let Some(env) = parsed.get("env") else {
                        return None;
                    };
                    let Some(env_table) = env.as_table() else {
                        return Some(CargoConfigOverrideFacts {
                            rel_path,
                            parse_error: Some(format!(
                                "invalid cargo config shape: `env` must be a table, found {}",
                                match env {
                                    toml::Value::String(_) => "string",
                                    toml::Value::Integer(_) => "integer",
                                    toml::Value::Float(_) => "float",
                                    toml::Value::Boolean(_) => "bool",
                                    toml::Value::Datetime(_) => "datetime",
                                    toml::Value::Array(_) => "array",
                                    toml::Value::Table(_) => "table",
                                }
                            )),
                        });
                    };
                    env_table
                        .get("CLIPPY_CONF_DIR")
                        .map(|_| CargoConfigOverrideFacts {
                            rel_path,
                            parse_error: None,
                        })
                }
                Ok(None) => None,
                Err(parse_error) => Some(CargoConfigOverrideFacts {
                    rel_path,
                    parse_error: Some(parse_error),
                }),
            }
        })
        .collect()
}

pub(super) fn push_coverage_facts(
    rel_dir: &str,
    kind: PolicyRootKind,
    allowed_configs: &[ClippyConfigFacts],
    covered_units: &mut Vec<CoveredRustUnitFacts>,
    uncovered_units: &mut Vec<UncoveredRustUnitFacts>,
) {
    if let Some(covering_config_rel) = nearest_covering_config(rel_dir, allowed_configs) {
        covered_units.push(CoveredRustUnitFacts {
            rel_dir: rel_dir.to_owned(),
            kind,
            covering_config_rel,
        });
    } else {
        uncovered_units.push(UncoveredRustUnitFacts {
            rel_dir: rel_dir.to_owned(),
            kind,
        });
    }
}

fn cargo_config_owner_is_relevant(owner_rel: &str, validation_scope: Option<&str>) -> bool {
    config_dir_is_relevant(owner_rel, validation_scope)
}

fn config_dir_is_relevant(rel_dir: &str, validation_scope: Option<&str>) -> bool {
    validation_scope.is_none_or(|scope_rel| paths_intersect_scope(rel_dir, scope_rel))
}

fn paths_intersect_scope(rel_dir: &str, scope_rel: &str) -> bool {
    path_is_under(rel_dir, scope_rel) || path_is_under(scope_rel, rel_dir)
}

fn path_is_under(rel_path: &str, parent_rel: &str) -> bool {
    parent_rel.is_empty()
        || rel_path == parent_rel
        || rel_path
            .strip_prefix(parent_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}

fn parse_config(
    tree: &ProjectTree,
    rel_dir: &str,
    rel_path: &str,
    policy_context_parse_error: Option<String>,
    profile_name: Option<String>,
    garde_enabled: bool,
    published_library_policy: bool,
) -> ClippyConfigFacts {
    match tree
        .file_content(rel_path)
        .map(toml::from_str::<toml::Value>)
    {
        Some(Ok(parsed)) => ClippyConfigFacts {
            rel_dir: rel_dir.to_owned(),
            rel_path: rel_path.to_owned(),
            parsed: Some(parsed),
            parse_error: None,
            policy_context_parse_error,
            profile_name,
            garde_enabled,
            published_library_policy,
        },
        Some(Err(err)) => ClippyConfigFacts {
            rel_dir: rel_dir.to_owned(),
            rel_path: rel_path.to_owned(),
            parsed: None,
            parse_error: Some(err.to_string()),
            policy_context_parse_error,
            profile_name,
            garde_enabled,
            published_library_policy,
        },
        None => ClippyConfigFacts {
            rel_dir: rel_dir.to_owned(),
            rel_path: rel_path.to_owned(),
            parsed: None,
            parse_error: Some("clippy.toml content missing from ProjectTree".to_owned()),
            policy_context_parse_error,
            profile_name,
            garde_enabled,
            published_library_policy,
        },
    }
}

fn nearest_covering_config(rel_dir: &str, allowed_configs: &[ClippyConfigFacts]) -> Option<String> {
    allowed_configs
        .iter()
        .filter(|config| is_ancestor_dir(&config.rel_dir, rel_dir))
        .max_by_key(|config| config.rel_dir.len())
        .map(|config| config.rel_path.clone())
}

fn is_ancestor_dir(ancestor: &str, rel_dir: &str) -> bool {
    ancestor.is_empty() || ancestor == rel_dir || rel_dir.starts_with(&format!("{ancestor}/"))
}
