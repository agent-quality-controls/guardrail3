use std::collections::{BTreeMap, BTreeSet};

use crate::domain::project_tree::ProjectTree;

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
    pub profile_name: Option<String>,
    pub garde_enabled: bool,
    pub package_publishable: bool,
}

#[derive(Debug, Clone)]
pub enum ForbiddenConfigReason {
    NotAllowedRoot,
    ShadowedSameRoot { preferred_rel_path: String },
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
    pub allowed_configs: Vec<ClippyConfigFacts>,
    pub forbidden_configs: Vec<ForbiddenConfigFacts>,
    pub covered_units: Vec<CoveredRustUnitFacts>,
    pub uncovered_units: Vec<UncoveredRustUnitFacts>,
}

#[derive(Debug, Clone)]
struct CargoRootFacts {
    rel_dir: String,
    has_workspace: bool,
    has_package: bool,
    workspace_members: Vec<String>,
}

#[derive(Debug, Clone)]
struct PolicySettings {
    profile_name: Option<String>,
    garde_enabled: bool,
}

pub fn collect(tree: &ProjectTree) -> ClippyFacts {
    let cargo_roots = collect_cargo_roots(tree);
    let workspace_roots: BTreeSet<_> = cargo_roots
        .values()
        .filter(|facts| facts.has_workspace)
        .map(|facts| facts.rel_dir.clone())
        .collect();
    let workspace_members: BTreeSet<_> = cargo_roots
        .values()
        .flat_map(|facts| facts.workspace_members.iter().cloned())
        .collect();
    let standalone_package_roots: BTreeSet<_> = cargo_roots
        .values()
        .filter(|facts| facts.has_package && !workspace_members.contains(&facts.rel_dir))
        .map(|facts| facts.rel_dir.clone())
        .collect();
    let policy_map = read_policy_map(tree, &cargo_roots, &standalone_package_roots);

    let mut allowed_policy_roots = BTreeSet::from([String::new()]);
    allowed_policy_roots.extend(workspace_roots.iter().cloned());
    allowed_policy_roots.extend(standalone_package_roots.iter().cloned());

    let configs = collect_configs(tree, &policy_map);
    let mut allowed_configs = Vec::new();
    let mut forbidden_configs = Vec::new();
    for config in configs {
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

    ClippyFacts {
        allowed_configs,
        forbidden_configs,
        covered_units,
        uncovered_units,
    }
}

fn config_precedence(rel_path: &str) -> usize {
    if rel_path.ends_with("clippy.toml") && !rel_path.ends_with(".clippy.toml") {
        return 0;
    }
    if rel_path.ends_with(".clippy.toml") {
        return 1;
    }
    2
}

fn collect_cargo_roots(tree: &ProjectTree) -> BTreeMap<String, CargoRootFacts> {
    let mut dirs = BTreeSet::new();
    if tree.file_exists("Cargo.toml") {
        let _ = dirs.insert(String::new());
    }
    dirs.extend(tree.dirs_with_file("Cargo.toml"));

    dirs.into_iter()
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
                    has_package: false,
                    workspace_members: Vec::new(),
                },
                |parsed| CargoRootFacts {
                    rel_dir: rel_dir.clone(),
                    has_workspace: parsed.get("workspace").is_some(),
                    has_package: parsed.get("package").is_some(),
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
    policy_map: &BTreeMap<String, PolicySettings>,
) -> Vec<ClippyConfigFacts> {
    let mut paths = Vec::new();
    for file_name in ["clippy.toml", ".clippy.toml"] {
        if tree.file_exists(file_name) {
            paths.push(("".to_owned(), file_name.to_owned()));
        }
        paths.extend(tree.dirs_with_file(file_name).into_iter().map(|rel_dir| {
            let rel_path = ProjectTree::join_rel(&rel_dir, file_name);
            (rel_dir, rel_path)
        }));
    }

    paths
        .into_iter()
        .map(|(rel_dir, rel_path)| {
            let settings = policy_settings_for(rel_dir.as_str(), policy_map);
            let package_publishable = package_publishable(tree, rel_dir.as_str());
            parse_config(
                tree,
                &rel_dir,
                &rel_path,
                settings.profile_name,
                settings.garde_enabled,
                package_publishable,
            )
        })
        .collect()
}

fn parse_config(
    tree: &ProjectTree,
    rel_dir: &str,
    rel_path: &str,
    profile_name: Option<String>,
    garde_enabled: bool,
    package_publishable: bool,
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
            profile_name,
            garde_enabled,
            package_publishable,
        },
        Some(Err(err)) => ClippyConfigFacts {
            rel_dir: rel_dir.to_owned(),
            rel_path: rel_path.to_owned(),
            parsed: None,
            parse_error: Some(err.to_string()),
            profile_name,
            garde_enabled,
            package_publishable,
        },
        None => ClippyConfigFacts {
            rel_dir: rel_dir.to_owned(),
            rel_path: rel_path.to_owned(),
            parsed: None,
            parse_error: Some("clippy.toml content missing from ProjectTree".to_owned()),
            profile_name,
            garde_enabled,
            package_publishable,
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

fn read_profile_name(tree: &ProjectTree) -> Option<String> {
    let content = tree.file_content("guardrail3.toml")?;
    let parsed = toml::from_str::<toml::Value>(content).ok()?;

    parsed
        .get("profile")
        .and_then(|value| value.get("name"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
}

fn read_policy_map(
    tree: &ProjectTree,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    standalone_package_roots: &BTreeSet<String>,
) -> BTreeMap<String, PolicySettings> {
    let mut map = BTreeMap::new();
    let default_profile = read_profile_name(tree);
    let default_garde = read_global_garde(tree);
    let _ = map.insert(
        String::new(),
        PolicySettings {
            profile_name: default_profile.clone(),
            garde_enabled: default_garde,
        },
    );

    let Some(content) = tree.file_content("guardrail3.toml") else {
        return map;
    };
    let Ok(parsed) = toml::from_str::<toml::Value>(content) else {
        return map;
    };
    let rust = parsed.get("rust");

    if let Some(apps) = rust
        .and_then(|value| value.get("apps"))
        .and_then(toml::Value::as_table)
    {
        let resolved_app_paths = resolve_app_paths(cargo_roots);
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

    map
}

fn resolve_app_paths(cargo_roots: &BTreeMap<String, CargoRootFacts>) -> BTreeMap<String, String> {
    crate::app::core::discover::resolve_app_paths_from_member_dirs(
        cargo_roots
            .values()
            .filter(|facts| facts.has_workspace)
            .flat_map(|workspace| workspace.workspace_members.iter().cloned())
            .collect::<Vec<_>>(),
    )
}

fn read_global_garde(tree: &ProjectTree) -> bool {
    let Some(content) = tree.file_content("guardrail3.toml") else {
        return true;
    };
    let Ok(parsed) = toml::from_str::<toml::Value>(content) else {
        return true;
    };

    parsed
        .get("rust")
        .and_then(|value| value.get("checks"))
        .and_then(|value| value.get("garde"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(true)
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
        Some(toml::Value::Array(array)) => !array.is_empty(),
        _ => true,
    }
}
