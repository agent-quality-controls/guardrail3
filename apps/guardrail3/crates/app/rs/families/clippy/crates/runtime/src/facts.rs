use std::collections::{BTreeMap, BTreeSet};

#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
use guardrail3_app_rs_family_mapper::RsClippyRoute;
use guardrail3_domain_project_tree::ProjectTree;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

#[cfg(test)]
use super::inputs::ConfigClippyInput;

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
    pub rel_dir: String,
    pub rel_path: String,
    pub parsed: Option<toml::Value>,
    pub parse_error: Option<String>,
    pub policy_context_parse_error: Option<String>,
    pub profile_name: Option<String>,
    pub garde_enabled: bool,
    pub published_library_policy: bool,
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
    pub config: ClippyConfigFacts,
    pub reason: ForbiddenConfigReason,
}

#[derive(Debug, Clone)]
pub struct CoveredRustUnitFacts {
    pub rel_dir: String,
    pub kind: PolicyRootKind,
    pub covering_config_rel: String,
}

#[derive(Debug, Clone)]
pub struct UncoveredRustUnitFacts {
    pub rel_dir: String,
    pub kind: PolicyRootKind,
}

#[derive(Debug, Clone)]
pub struct ClippyFacts {
    pub policy_context_parse_error: Option<String>,
    pub allowed_configs: Vec<ClippyConfigFacts>,
    pub forbidden_configs: Vec<ForbiddenConfigFacts>,
    pub cargo_config_overrides: Vec<CargoConfigOverrideFacts>,
    pub cargo_root_failures: Vec<CargoRootFailureFacts>,
    pub covered_units: Vec<CoveredRustUnitFacts>,
    pub uncovered_units: Vec<UncoveredRustUnitFacts>,
}

#[derive(Debug, Clone)]
pub struct CargoConfigOverrideFacts {
    pub rel_path: String,
    pub parse_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CargoRootFailureFacts {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub parse_error: String,
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
    let routed_root_rels = route
        .roots
        .iter()
        .map(|root| root.rel_dir.clone())
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
        collect_cargo_config_overrides(tree, &routed_root_rels, &cargo_roots);

    let mut allowed_policy_roots = BTreeSet::new();
    let _ = allowed_policy_roots.insert(String::new());
    allowed_policy_roots.extend(workspace_roots.iter().cloned());
    allowed_policy_roots.extend(standalone_package_roots.iter().cloned());

    let configs = collect_configs(tree, &cargo_roots, &policy_map, &routed_root_rels);
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
                .map(|parse_error| CargoRootFailureFacts {
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
    collect(tree, &family_route_for_tests(tree))
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
fn family_route_for_tests(tree: &ProjectTree) -> RsClippyRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let selected = RustFamilySelection::new(std::collections::BTreeSet::from([
        RustValidateFamily::Clippy,
    ]));
    FamilyMapper::new(tree, &scope, None, &selected, None).map_rs_clippy()
}

fn config_precedence(rel_path: &str) -> usize {
    if rel_path.ends_with(".clippy.toml") {
        return 0;
    }
    if rel_path.ends_with("clippy.toml") && !rel_path.ends_with(".clippy.toml") {
        return 1;
    }
    2
}

fn collect_cargo_roots(
    tree: &ProjectTree,
    route: &RsClippyRoute,
) -> BTreeMap<String, CargoRootFacts> {
    route
        .roots
        .iter()
        .map(|root| (root.rel_dir.clone(), root.cargo_rel_path.clone()))
        .map(|(rel_dir, cargo_rel_path)| {
            let facts = match tree.file_content(&cargo_rel_path) {
                Some(content) => match toml::from_str::<toml::Value>(content) {
                    Ok(parsed) => CargoRootFacts {
                        rel_dir: rel_dir.clone(),
                        cargo_rel_path: cargo_rel_path.clone(),
                        parse_error: None,
                        has_workspace: parsed.get("workspace").is_some(),
                        has_package: parsed.get("package").is_some(),
                        workspace_members: parse_workspace_members(tree, &rel_dir, &parsed),
                    },
                    Err(err) => CargoRootFacts {
                        rel_dir: rel_dir.clone(),
                        cargo_rel_path: cargo_rel_path.clone(),
                        parse_error: Some(err.to_string()),
                        has_workspace: false,
                        has_package: false,
                        workspace_members: Vec::new(),
                    },
                },
                None => CargoRootFacts {
                    rel_dir: rel_dir.clone(),
                    cargo_rel_path: cargo_rel_path.clone(),
                    parse_error: Some("Cargo.toml content missing from ProjectTree".to_owned()),
                    has_workspace: false,
                    has_package: false,
                    workspace_members: Vec::new(),
                },
            };
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
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    policy_map: &ResolvedPolicyMap,
    routed_root_rels: &BTreeSet<String>,
) -> Vec<ClippyConfigFacts> {
    let mut paths = Vec::new();
    for file_name in ["clippy.toml", ".clippy.toml"] {
        if tree.file_exists(file_name) {
            paths.push(("".to_owned(), file_name.to_owned()));
        }
        paths.extend(
            tree.dirs_with_file(file_name)
                .into_iter()
                .filter(|rel_dir| is_under_routed_root(rel_dir, routed_root_rels))
                .map(|rel_dir| {
                    let rel_path = ProjectTree::join_rel(&rel_dir, file_name);
                    (rel_dir, rel_path)
                }),
        );
    }

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

fn collect_cargo_config_overrides(
    tree: &ProjectTree,
    routed_root_rels: &BTreeSet<String>,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
) -> Vec<CargoConfigOverrideFacts> {
    let mut rel_paths = Vec::new();

    for rel_path in [".cargo/config.toml", ".cargo/config"] {
        if tree.file_exists(rel_path) {
            rel_paths.push(rel_path.to_owned());
        }
    }

    rel_paths.extend(
        tree.dirs_with_file("config.toml")
            .into_iter()
            .filter(|rel_dir| rel_dir.ends_with("/.cargo"))
            .filter(|rel_dir| {
                cargo_config_applies_to_routed_roots(rel_dir, routed_root_rels, cargo_roots)
            })
            .map(|rel_dir| ProjectTree::join_rel(&rel_dir, "config.toml")),
    );
    rel_paths.extend(
        tree.dirs_with_file("config")
            .into_iter()
            .filter(|rel_dir| rel_dir.ends_with("/.cargo"))
            .filter(|rel_dir| {
                cargo_config_applies_to_routed_roots(rel_dir, routed_root_rels, cargo_roots)
            })
            .map(|rel_dir| ProjectTree::join_rel(&rel_dir, "config")),
    );

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

fn cargo_config_applies_to_routed_roots(
    rel_dir: &str,
    routed_root_rels: &BTreeSet<String>,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
) -> bool {
    let owner_rel = rel_dir.strip_suffix("/.cargo").unwrap_or(rel_dir);
    owner_rel.is_empty()
        || cargo_roots.keys().any(|cargo_root_rel| {
            is_under_routed_root(cargo_root_rel, routed_root_rels)
                && (cargo_root_rel == owner_rel
                    || cargo_root_rel.starts_with(&format!("{owner_rel}/")))
        })
}

fn is_under_routed_root(rel_dir: &str, routed_root_rels: &BTreeSet<String>) -> bool {
    routed_root_rels.iter().any(|root_rel| {
        root_rel.is_empty() || rel_dir == root_rel || rel_dir.starts_with(&format!("{root_rel}/"))
    })
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

fn push_coverage_facts(
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

fn read_policy_map(
    tree: &ProjectTree,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    standalone_package_roots: &BTreeSet<String>,
) -> ResolvedPolicyMap {
    let mut map = BTreeMap::new();
    let guardrail = read_guardrail_policy(tree);
    let default_profile = guardrail.default_profile.clone();
    let default_garde = guardrail.default_garde;
    let resolved_app_paths = resolve_app_paths(cargo_roots);
    let app_root_paths: BTreeSet<_> = resolved_app_paths.values().cloned().collect();
    let _ = map.insert(
        String::new(),
        PolicySettings {
            profile_name: default_profile.clone(),
            garde_enabled: default_garde,
        },
    );

    let Some(parsed) = guardrail.parsed.as_ref() else {
        return ResolvedPolicyMap {
            map,
            parse_error: guardrail.parse_error,
        };
    };
    let rust = parsed.get("rust");

    if let Some(apps) = rust
        .and_then(|value| value.get("apps"))
        .and_then(toml::Value::as_table)
    {
        for (app_name, app_cfg) in apps {
            let profile_name = app_cfg
                .get("type")
                .or_else(|| app_cfg.get("profile"))
                .and_then(toml::Value::as_str)
                .map(str::to_owned)
                .or_else(|| default_profile.clone());
            let garde_enabled = app_cfg
                .get("checks")
                .and_then(|value| value.get("garde"))
                .and_then(toml::Value::as_bool)
                .unwrap_or(default_garde);
            if let Some(rel_dir) = resolved_app_paths.get(app_name) {
                let _ = map.insert(
                    rel_dir.clone(),
                    PolicySettings {
                        profile_name: profile_name.clone(),
                        garde_enabled,
                    },
                );
            }
        }
    }

    if let Some(packages) = rust.and_then(|value| value.get("packages")) {
        let profile_name = packages
            .get("type")
            .or_else(|| packages.get("profile"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
            .or_else(|| Some("library".to_owned()))
            .or_else(|| default_profile.clone());
        let garde_enabled = packages
            .get("checks")
            .and_then(|value| value.get("garde"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(default_garde);
        if !resolved_app_paths
            .values()
            .any(|rel_dir| rel_dir.is_empty())
        {
            let _ = map.insert(
                String::new(),
                PolicySettings {
                    profile_name: profile_name.clone(),
                    garde_enabled,
                },
            );
        }
        for rel_dir in cargo_roots
            .values()
            .filter(|facts| facts.has_workspace)
            .map(|facts| facts.rel_dir.as_str())
            .filter(|rel_dir| !rel_dir.is_empty())
            .filter(|rel_dir| !app_root_paths.contains(*rel_dir))
        {
            let _ = map.insert(
                rel_dir.to_owned(),
                PolicySettings {
                    profile_name: profile_name.clone(),
                    garde_enabled,
                },
            );
        }
        for rel_dir in standalone_package_roots {
            let _ = map.insert(
                rel_dir.clone(),
                PolicySettings {
                    profile_name: profile_name.clone(),
                    garde_enabled,
                },
            );
        }
    }

    ResolvedPolicyMap {
        map,
        parse_error: guardrail.parse_error,
    }
}

fn resolve_app_paths(cargo_roots: &BTreeMap<String, CargoRootFacts>) -> BTreeMap<String, String> {
    let mut resolved = guardrail3_app_core::discover::resolve_app_paths_from_member_dirs(
        cargo_roots
            .values()
            .filter(|facts| facts.has_workspace)
            .flat_map(|workspace| workspace.workspace_members.iter().cloned())
            .collect::<Vec<_>>(),
    );

    for rel_dir in cargo_roots.keys() {
        let mut parts = rel_dir.split('/');
        if let (Some("apps"), Some(app_name), None) = (parts.next(), parts.next(), parts.next()) {
            let _ = resolved
                .entry(app_name.to_owned())
                .or_insert_with(|| rel_dir.clone());
        }
    }

    resolved
}

fn read_guardrail_policy(tree: &ProjectTree) -> GuardrailPolicyFacts {
    if !tree.file_exists("guardrail3.toml") {
        return GuardrailPolicyFacts {
            parsed: None,
            parse_error: None,
            default_profile: None,
            default_garde: true,
        };
    }

    let Some(content) = tree.file_content("guardrail3.toml") else {
        return GuardrailPolicyFacts {
            parsed: None,
            parse_error: Some("guardrail3.toml content missing from ProjectTree".to_owned()),
            default_profile: None,
            default_garde: true,
        };
    };

    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => match validate_guardrail_policy_shape(&parsed) {
            Ok(()) => GuardrailPolicyFacts {
                default_profile: parsed
                    .get("profile")
                    .and_then(|value| value.get("name"))
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned),
                default_garde: parsed
                    .get("rust")
                    .and_then(|value| value.get("checks"))
                    .and_then(|value| value.get("garde"))
                    .and_then(toml::Value::as_bool)
                    .unwrap_or(true),
                parsed: Some(parsed),
                parse_error: None,
            },
            Err(err) => GuardrailPolicyFacts {
                parsed: None,
                parse_error: Some(err),
                default_profile: None,
                default_garde: true,
            },
        },
        Err(err) => GuardrailPolicyFacts {
            parsed: None,
            parse_error: Some(err.to_string()),
            default_profile: None,
            default_garde: true,
        },
    }
}

fn validate_guardrail_policy_shape(parsed: &toml::Value) -> Result<(), String> {
    if let Some(profile) = parsed.get("profile") {
        let table = profile
            .as_table()
            .ok_or_else(|| "`profile` must be a table in active `guardrail3.toml`.".to_owned())?;
        if let Some(name) = table.get("name") {
            if !name.is_str() {
                return Err(
                    "`profile.name` must be a string in active `guardrail3.toml`.".to_owned(),
                );
            }
        }
    }

    let Some(rust) = parsed.get("rust") else {
        return Ok(());
    };
    let rust_table = rust
        .as_table()
        .ok_or_else(|| "`rust` must be a table in active `guardrail3.toml`.".to_owned())?;

    if let Some(checks) = rust_table.get("checks") {
        validate_garde_field(checks, "`rust.checks`")?;
    }

    if let Some(apps) = rust_table.get("apps") {
        let apps_table = apps
            .as_table()
            .ok_or_else(|| "`rust.apps` must be a table in active `guardrail3.toml`.".to_owned())?;
        for (app_name, app_cfg) in apps_table {
            let ctx = format!("`rust.apps.{app_name}`");
            validate_profile_block(app_cfg, &ctx)?;
        }
    }

    if let Some(packages) = rust_table.get("packages") {
        validate_profile_block(packages, "`rust.packages`")?;
    }

    Ok(())
}

fn validate_profile_block(value: &toml::Value, context: &str) -> Result<(), String> {
    let table = value
        .as_table()
        .ok_or_else(|| format!("{context} must be a table in active `guardrail3.toml`."))?;
    if let Some(profile_name) = table.get("type").or_else(|| table.get("profile")) {
        if !profile_name.is_str() {
            return Err(format!(
                "{context}.type/profile must be a string in active `guardrail3.toml`."
            ));
        }
    }
    if let Some(checks) = table.get("checks") {
        validate_garde_field(checks, &format!("{context}.checks"))?;
    }
    Ok(())
}

fn validate_garde_field(value: &toml::Value, context: &str) -> Result<(), String> {
    let table = value
        .as_table()
        .ok_or_else(|| format!("{context} must be a table in active `guardrail3.toml`."))?;
    if let Some(garde) = table.get("garde") {
        if !garde.is_bool() {
            return Err(format!(
                "{context}.garde must be a bool in active `guardrail3.toml`."
            ));
        }
    }
    Ok(())
}

fn policy_settings_for(
    rel_dir: &str,
    policy_map: &BTreeMap<String, PolicySettings>,
) -> PolicySettings {
    if rel_dir.is_empty() {
        return policy_map.get("").cloned().unwrap_or(PolicySettings {
            profile_name: None,
            garde_enabled: true,
        });
    }

    if let Some(settings) = policy_map.get(rel_dir) {
        return settings.clone();
    }

    policy_map.get("").cloned().unwrap_or(PolicySettings {
        profile_name: None,
        garde_enabled: true,
    })
}

fn published_library_policy(
    tree: &ProjectTree,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    rel_dir: &str,
    profile_name: Option<&str>,
) -> bool {
    if profile_name != Some("library") {
        return false;
    }

    package_publishable(tree, rel_dir)
        || cargo_roots
            .get(rel_dir)
            .filter(|facts| facts.has_workspace)
            .map(|facts| {
                facts
                    .workspace_members
                    .iter()
                    .any(|member_rel| package_publishable(tree, member_rel))
            })
            .unwrap_or(false)
}

fn package_publishable(tree: &ProjectTree, rel_dir: &str) -> bool {
    let cargo_rel = if rel_dir.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        ProjectTree::join_rel(rel_dir, "Cargo.toml")
    };
    let Some(content) = tree.file_content(&cargo_rel) else {
        return false;
    };
    let Ok(parsed) = toml::from_str::<toml::Value>(content) else {
        return false;
    };
    let Some(package) = parsed.get("package") else {
        return false;
    };
    match package.get("publish") {
        None => true,
        Some(toml::Value::Boolean(value)) => *value,
        Some(toml::Value::Array(array)) => {
            !array.is_empty() && array.iter().all(toml::Value::is_str)
        }
        _ => false,
    }
}

#[cfg(test)]
#[path = "facts_tests/mod.rs"] // reason: test-only sidecar module wiring
mod facts_tests;
