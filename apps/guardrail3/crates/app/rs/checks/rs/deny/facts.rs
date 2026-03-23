use std::collections::{BTreeMap, BTreeSet};

use crate::domain::project_tree::ProjectTree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyRootKind {
    ValidationRoot,
    WorkspaceRoot,
    StandalonePackageRoot,
}

impl PolicyRootKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ValidationRoot => "validation root",
            Self::WorkspaceRoot => "workspace root",
            Self::StandalonePackageRoot => "standalone package root",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DenyConfigFacts {
    pub policy_root_rel: String,
    pub rel_path: String,
    pub file_kind: String,
    pub parsed: Option<toml::Value>,
    pub parse_error: Option<String>,
    pub profile_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ForbiddenDenyConfigFacts {
    pub policy_root_rel: String,
    pub rel_path: String,
    pub file_kind: String,
    pub parse_error: Option<String>,
    pub shadowed_root_rel: Option<String>,
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
pub struct DenyFacts {
    pub allowed_configs: Vec<DenyConfigFacts>,
    pub forbidden_configs: Vec<ForbiddenDenyConfigFacts>,
    pub same_root_conflicts: Vec<SameRootConflictFacts>,
    pub covered_units: Vec<CoveredRustUnitFacts>,
    pub uncovered_units: Vec<UncoveredRustUnitFacts>,
}

#[derive(Debug, Clone)]
pub struct SameRootConflictFacts {
    pub policy_root_rel: String,
    pub rel_paths: Vec<String>,
}

#[derive(Debug, Clone)]
struct CargoRootFacts {
    rel_dir: String,
    has_workspace: bool,
    has_package: bool,
    workspace_members: Vec<String>,
}

pub fn collect(tree: &ProjectTree) -> DenyFacts {
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

    let mut allowed_policy_roots = BTreeSet::from([String::new()]);
    allowed_policy_roots.extend(workspace_roots.iter().cloned());
    allowed_policy_roots.extend(standalone_package_roots.iter().cloned());

    let profile_map = read_profile_map(tree, &cargo_roots, &standalone_package_roots);

    let configs = collect_configs(tree, &profile_map);
    let mut allowed_configs = Vec::new();
    let mut forbidden_configs = Vec::new();
    for config in configs {
        if allowed_policy_roots.contains(&config.policy_root_rel) {
            allowed_configs.push(config);
        } else {
            let shadowed_root_rel =
                nearest_allowed_ancestor(&config.policy_root_rel, &allowed_policy_roots);
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
    push_coverage_facts(
        "",
        PolicyRootKind::ValidationRoot,
        &allowed_configs,
        &mut covered_units,
        &mut uncovered_units,
    );
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

    allowed_configs.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    forbidden_configs.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    same_root_conflicts.sort_by(|a, b| a.policy_root_rel.cmp(&b.policy_root_rel));
    covered_units.sort_by(|a, b| a.rel_dir.cmp(&b.rel_dir));
    uncovered_units.sort_by(|a, b| a.rel_dir.cmp(&b.rel_dir));

    DenyFacts {
        allowed_configs,
        forbidden_configs,
        same_root_conflicts,
        covered_units,
        uncovered_units,
    }
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
    profile_map: &BTreeMap<String, Option<String>>,
) -> Vec<DenyConfigFacts> {
    let mut configs = Vec::new();
    let mut seen_paths = BTreeSet::new();
    push_config_if_present(
        tree,
        "",
        "deny.toml",
        "deny.toml",
        profile_map,
        &mut seen_paths,
        &mut configs,
    );
    push_config_if_present(
        tree,
        "",
        ".deny.toml",
        ".deny.toml",
        profile_map,
        &mut seen_paths,
        &mut configs,
    );
    push_config_if_present(
        tree,
        "",
        ".cargo/deny.toml",
        ".cargo/deny.toml",
        profile_map,
        &mut seen_paths,
        &mut configs,
    );

    for rel_dir in tree.dirs_with_file("deny.toml") {
        if rel_dir == ".cargo" || rel_dir.ends_with("/.cargo") {
            let policy_root_rel = parent_dir(&rel_dir);
            let rel_path = ProjectTree::join_rel(&rel_dir, "deny.toml");
            push_config_if_present(
                tree,
                &policy_root_rel,
                &rel_path,
                ".cargo/deny.toml",
                profile_map,
                &mut seen_paths,
                &mut configs,
            );
        } else {
            let rel_path = ProjectTree::join_rel(&rel_dir, "deny.toml");
            push_config_if_present(
                tree,
                &rel_dir,
                &rel_path,
                "deny.toml",
                profile_map,
                &mut seen_paths,
                &mut configs,
            );
        }
    }

    for rel_dir in tree.dirs_with_file(".deny.toml") {
        let rel_path = ProjectTree::join_rel(&rel_dir, ".deny.toml");
        push_config_if_present(
            tree,
            &rel_dir,
            &rel_path,
            ".deny.toml",
            profile_map,
            &mut seen_paths,
            &mut configs,
        );
    }

    configs
}

fn push_config_if_present(
    tree: &ProjectTree,
    policy_root_rel: &str,
    rel_path: &str,
    file_kind: &str,
    profile_map: &BTreeMap<String, Option<String>>,
    seen_paths: &mut BTreeSet<String>,
    configs: &mut Vec<DenyConfigFacts>,
) {
    if !seen_paths.insert(rel_path.to_owned()) {
        return;
    }
    let Some(content) = tree.file_content(rel_path) else {
        return;
    };
    let profile_name = profile_for(policy_root_rel, profile_map);
    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => configs.push(DenyConfigFacts {
            policy_root_rel: policy_root_rel.to_owned(),
            rel_path: rel_path.to_owned(),
            file_kind: file_kind.to_owned(),
            parsed: Some(parsed),
            parse_error: None,
            profile_name,
        }),
        Err(err) => configs.push(DenyConfigFacts {
            policy_root_rel: policy_root_rel.to_owned(),
            rel_path: rel_path.to_owned(),
            file_kind: file_kind.to_owned(),
            parsed: None,
            parse_error: Some(err.to_string()),
            profile_name,
        }),
    }
}

fn push_coverage_facts(
    rel_dir: &str,
    kind: PolicyRootKind,
    allowed_configs: &[DenyConfigFacts],
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

fn nearest_covering_config(rel_dir: &str, allowed_configs: &[DenyConfigFacts]) -> Option<String> {
    allowed_configs
        .iter()
        .filter(|config| is_ancestor_dir(&config.policy_root_rel, rel_dir))
        .max_by_key(|config| {
            (
                config.policy_root_rel.len(),
                config_precedence(&config.file_kind),
            )
        })
        .map(|config| config.rel_path.clone())
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
            if rel_paths.len() <= 1 {
                None
            } else {
                rel_paths.sort();
                Some(SameRootConflictFacts {
                    policy_root_rel,
                    rel_paths,
                })
            }
        })
        .collect()
}

fn nearest_allowed_ancestor(rel_dir: &str, allowed_roots: &BTreeSet<String>) -> Option<String> {
    allowed_roots
        .iter()
        .filter(|ancestor| is_ancestor_dir(ancestor, rel_dir))
        .max_by_key(|ancestor| ancestor.len())
        .cloned()
}

fn is_ancestor_dir(ancestor: &str, rel_dir: &str) -> bool {
    ancestor.is_empty() || ancestor == rel_dir || rel_dir.starts_with(&format!("{ancestor}/"))
}

fn parent_dir(rel_dir: &str) -> String {
    rel_dir
        .rsplit_once('/')
        .map_or_else(String::new, |(parent, _)| parent.to_owned())
}

fn config_precedence(file_kind: &str) -> usize {
    match file_kind {
        "deny.toml" => 3,
        ".deny.toml" => 2,
        ".cargo/deny.toml" => 1,
        _ => 0,
    }
}

fn read_profile_map(
    tree: &ProjectTree,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    standalone_package_roots: &BTreeSet<String>,
) -> BTreeMap<String, Option<String>> {
    let mut map = BTreeMap::new();
    let default_profile = read_default_profile(tree);
    let _ = map.insert(String::new(), default_profile.clone());

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
            if let Some(rel_dir) = resolved_app_paths.get(app_name) {
                let _ = map.insert(rel_dir.clone(), profile_name);
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
        for rel_dir in standalone_package_roots {
            let _ = map.insert(rel_dir.clone(), profile_name.clone());
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

fn read_default_profile(tree: &ProjectTree) -> Option<String> {
    let content = tree.file_content("guardrail3.toml")?;
    let parsed = toml::from_str::<toml::Value>(content).ok()?;
    parsed
        .get("profile")
        .and_then(|value| value.get("name"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
}

fn profile_for(rel_dir: &str, profile_map: &BTreeMap<String, Option<String>>) -> Option<String> {
    if let Some(profile) = profile_map.get(rel_dir) {
        return profile.clone();
    }
    profile_map.get("").cloned().flatten()
}
