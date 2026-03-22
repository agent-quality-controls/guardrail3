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
    pub forbidden_configs: Vec<ClippyConfigFacts>,
    pub covered_units: Vec<CoveredRustUnitFacts>,
    pub uncovered_units: Vec<UncoveredRustUnitFacts>,
    pub profile_name: Option<String>,
}

#[derive(Debug, Clone)]
struct CargoRootFacts {
    rel_dir: String,
    has_workspace: bool,
    has_package: bool,
    workspace_members: Vec<String>,
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

    let mut allowed_policy_roots = BTreeSet::from([String::new()]);
    allowed_policy_roots.extend(workspace_roots.iter().cloned());
    allowed_policy_roots.extend(standalone_package_roots.iter().cloned());

    let configs = collect_configs(tree);
    let mut allowed_configs = Vec::new();
    let mut forbidden_configs = Vec::new();
    for config in configs {
        if allowed_policy_roots.contains(&config.rel_dir) {
            allowed_configs.push(config);
        } else {
            forbidden_configs.push(config);
        }
    }

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
    forbidden_configs.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));

    ClippyFacts {
        allowed_configs,
        forbidden_configs,
        covered_units,
        uncovered_units,
        profile_name: read_profile_name(tree),
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

fn collect_configs(tree: &ProjectTree) -> Vec<ClippyConfigFacts> {
    let root = tree
        .file_exists("clippy.toml")
        .then(|| parse_config(tree, "", "clippy.toml"));
    let extras = tree
        .dirs_with_file("clippy.toml")
        .into_iter()
        .map(|rel_dir| {
            let rel_path = ProjectTree::join_rel(&rel_dir, "clippy.toml");
            parse_config(tree, &rel_dir, &rel_path)
        });

    root.into_iter().chain(extras).collect()
}

fn parse_config(tree: &ProjectTree, rel_dir: &str, rel_path: &str) -> ClippyConfigFacts {
    match tree
        .file_content(rel_path)
        .map(toml::from_str::<toml::Value>)
    {
        Some(Ok(parsed)) => ClippyConfigFacts {
            rel_dir: rel_dir.to_owned(),
            rel_path: rel_path.to_owned(),
            parsed: Some(parsed),
            parse_error: None,
        },
        Some(Err(err)) => ClippyConfigFacts {
            rel_dir: rel_dir.to_owned(),
            rel_path: rel_path.to_owned(),
            parsed: None,
            parse_error: Some(err.to_string()),
        },
        None => ClippyConfigFacts {
            rel_dir: rel_dir.to_owned(),
            rel_path: rel_path.to_owned(),
            parsed: None,
            parse_error: Some("clippy.toml content missing from ProjectTree".to_owned()),
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
