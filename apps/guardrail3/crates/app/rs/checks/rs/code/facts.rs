use std::collections::{BTreeMap, BTreeSet};

use crate::domain::project_tree::ProjectTree;

use super::discover::{cargo_toml_rels, is_test_path, rust_file_rels};

#[derive(Debug, Clone)]
pub struct RustCodeFileFacts {
    pub rel_path: String,
    pub is_test: bool,
    pub profile_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UnsafeCodeLintFacts {
    pub cargo_rel_path: String,
    pub lint_level: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CodeFacts {
    pub files: Vec<RustCodeFileFacts>,
    pub unsafe_code_lints: Vec<UnsafeCodeLintFacts>,
    pub exception_comments: Vec<ExceptionCommentFacts>,
}

pub fn collect(tree: &ProjectTree) -> CodeFacts {
    let cargo_roots = collect_cargo_roots(tree);
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

    let files = rust_file_rels(tree)
        .into_iter()
        .map(|rel_path| RustCodeFileFacts {
            profile_name: policy_settings_for(file_parent_rel(&rel_path), &policy_map).profile_name,
            is_test: is_test_path(&rel_path),
            rel_path,
        })
        .collect();

    let unsafe_code_lints = cargo_toml_rels(tree)
        .into_iter()
        .filter_map(|cargo_rel_path| {
            let parsed = tree
                .file_content(&cargo_rel_path)
                .and_then(|content| toml::from_str::<toml::Value>(content).ok())?;
            if parsed.get("workspace").is_none() {
                return None;
            }
            let lint_level = parsed
                .get("workspace")
                .and_then(|workspace| workspace.get("lints"))
                .and_then(|lints| lints.get("rust"))
                .and_then(|rust| rust.get("unsafe_code"))
                .and_then(toml::Value::as_str)
                .map(str::to_owned);
            Some(UnsafeCodeLintFacts {
                cargo_rel_path,
                lint_level,
            })
        })
        .collect();

    let exception_comments = collect_exception_comments(tree);

    CodeFacts {
        files,
        unsafe_code_lints,
        exception_comments,
    }
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
}

#[derive(Debug, Clone)]
pub struct ExceptionCommentFacts {
    pub rel_path: String,
    pub line: usize,
    pub line_text: String,
}

fn collect_exception_comments(tree: &ProjectTree) -> Vec<ExceptionCommentFacts> {
    let config_rels = [
        "clippy.toml",
        "deny.toml",
        "Cargo.toml",
        "rustfmt.toml",
        "rust-toolchain.toml",
    ];
    let mut comments = Vec::new();

    for rel_path in config_rels {
        let Some(content) = tree.file_content(rel_path) else {
            continue;
        };
        for (index, line) in content.lines().enumerate() {
            let upper = line.to_ascii_uppercase();
            if upper.contains("// EXCEPTION:") || upper.contains("# EXCEPTION:") {
                comments.push(ExceptionCommentFacts {
                    rel_path: rel_path.to_owned(),
                    line: index.saturating_add(1),
                    line_text: line.trim().to_owned(),
                });
            }
        }
    }

    comments
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
    let _ = map.insert(
        String::new(),
        PolicySettings {
            profile_name: default_profile.clone(),
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
            if let Some(rel_dir) = resolved_app_paths.get(app_name) {
                let _ = map.insert(
                    rel_dir.clone(),
                    PolicySettings {
                        profile_name: profile_name.clone(),
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
        for rel_dir in standalone_package_roots {
            let _ = map.insert(
                rel_dir.clone(),
                PolicySettings {
                    profile_name: profile_name.clone(),
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

fn policy_settings_for(
    rel_dir: &str,
    policy_map: &BTreeMap<String, PolicySettings>,
) -> PolicySettings {
    if rel_dir.is_empty() {
        return policy_map.get("").cloned().unwrap_or(PolicySettings { profile_name: None });
    }

    let mut current = rel_dir;
    loop {
        if let Some(settings) = policy_map.get(current) {
            return settings.clone();
        }
        let Some((parent, _)) = current.rsplit_once('/') else {
            break;
        };
        current = parent;
    }

    policy_map.get("").cloned().unwrap_or(PolicySettings { profile_name: None })
}

fn file_parent_rel(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
}
