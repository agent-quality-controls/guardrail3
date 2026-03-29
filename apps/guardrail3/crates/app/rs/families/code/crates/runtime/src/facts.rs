use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsCodeRoute;
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;

use super::discover::{is_test_root_path, rust_file_rels};

#[derive(Debug, Clone)]
pub struct RustCodeFileFacts {
    pub rel_path: String,
    pub is_test_root: bool,
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
    pub input_failures: Vec<CodeInputFailureFacts>,
}

pub fn collect(tree: &ProjectTree, route: &RsCodeRoute) -> CodeFacts {
    let mut input_failures = Vec::new();
    let active_root_dirs = active_root_dirs(route);
    let cargo_roots = collect_cargo_roots(tree, route, &active_root_dirs, &mut input_failures);
    let root_dirs = cargo_roots.keys().cloned().collect::<Vec<_>>();
    let policy_map = read_policy_map(tree, &cargo_roots, &mut input_failures);

    let files = rust_file_rels(tree)
        .into_iter()
        .filter(|rel_path| owning_root_dir(rel_path, &root_dirs).is_some())
        .map(|rel_path| RustCodeFileFacts {
            profile_name: policy_settings_for(file_parent_rel(&rel_path), &policy_map).profile_name,
            is_test_root: is_test_root_path(&rel_path),
            rel_path,
        })
        .collect();

    let mut unsafe_code_lints = Vec::new();
    for root in cargo_roots.values().filter(|root| root.has_workspace) {
        let Some(content) = tree.file_content(&root.cargo_rel_path) else {
            continue;
        };
        let parsed = match toml::from_str::<toml::Value>(content) {
            Ok(parsed) => parsed,
            Err(parse_error) => {
                input_failures.push(CodeInputFailureFacts {
                    rel_path: root.cargo_rel_path.clone(),
                    message: format!(
                        "Failed to parse Cargo.toml for code-family context: {parse_error}"
                    ),
                });
                continue;
            }
        };
        if parsed.get("workspace").is_none() {
            continue;
        }
        let lint_level = parsed
            .get("workspace")
            .and_then(|workspace| workspace.get("lints"))
            .and_then(|lints| lints.get("rust"))
            .and_then(|rust| rust.get("unsafe_code"))
            .and_then(parse_lint_level);
        unsafe_code_lints.push(UnsafeCodeLintFacts {
            cargo_rel_path: root.cargo_rel_path.clone(),
            lint_level,
        });
    }

    let exception_comments = collect_exception_comments(tree, &root_dirs);

    CodeFacts {
        files,
        unsafe_code_lints,
        exception_comments,
        input_failures,
    }
}

#[derive(Debug, Clone)]
struct CargoRootFacts {
    rel_dir: String,
    cargo_rel_path: String,
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

#[derive(Debug, Clone)]
pub struct CodeInputFailureFacts {
    pub rel_path: String,
    pub message: String,
}

fn collect_exception_comments(
    tree: &ProjectTree,
    root_dirs: &[String],
) -> Vec<ExceptionCommentFacts> {
    let mut comments = Vec::new();

    for rel_path in config_comment_rels(tree) {
        if owning_root_dir(&rel_path, root_dirs).is_none() {
            continue;
        }
        let Some(content) = tree.file_content(&rel_path) else {
            continue;
        };
        for (index, line) in content.lines().enumerate() {
            let Some(comment_text) = extract_exception_comment(line) else {
                continue;
            };
            let normalized = comment_text
                .trim_start_matches('#')
                .trim_start_matches('/')
                .trim_start();
            if normalized.to_ascii_uppercase().starts_with("EXCEPTION:") {
                comments.push(ExceptionCommentFacts {
                    rel_path: rel_path.clone(),
                    line: index.saturating_add(1),
                    line_text: comment_text.to_owned(),
                });
            }
        }
    }

    comments
}

fn parse_lint_level(value: &toml::Value) -> Option<String> {
    value.as_str().map(str::to_owned).or_else(|| {
        value
            .as_table()
            .and_then(|table| table.get("level"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
    })
}

fn extract_exception_comment(line: &str) -> Option<&str> {
    #[derive(Clone, Copy)]
    enum State {
        Normal,
        DoubleQuoted { escaped: bool },
        SingleQuoted,
    }

    let bytes = line.as_bytes();
    let mut index = 0usize;
    let mut state = State::Normal;

    while index < bytes.len() {
        match state {
            State::Normal => {
                if bytes[index] == b'#' {
                    return line.get(index..).map(str::trim_start);
                }
                if bytes[index] == b'/' && bytes.get(index.saturating_add(1)) == Some(&b'/') {
                    return line.get(index..).map(str::trim_start);
                }
                if bytes[index] == b'"' {
                    state = State::DoubleQuoted { escaped: false };
                } else if bytes[index] == b'\'' {
                    state = State::SingleQuoted;
                }
            }
            State::DoubleQuoted { escaped } => {
                if escaped {
                    state = State::DoubleQuoted { escaped: false };
                } else if bytes[index] == b'\\' {
                    state = State::DoubleQuoted { escaped: true };
                } else if bytes[index] == b'"' {
                    state = State::Normal;
                }
            }
            State::SingleQuoted => {
                if bytes[index] == b'\'' {
                    state = State::Normal;
                }
            }
        }
        index = index.saturating_add(1);
    }

    None
}

fn config_comment_rels(tree: &ProjectTree) -> Vec<String> {
    let config_names = [
        "guardrail3.toml",
        "clippy.toml",
        ".clippy.toml",
        "deny.toml",
        ".deny.toml",
        "Cargo.toml",
        "rustfmt.toml",
        "rust-toolchain.toml",
        "rust-toolchain",
    ];
    let mut rels = BTreeSet::new();

    for (dir_rel, entry) in &tree.structure {
        for file_name in &entry.files {
            if config_names.contains(&file_name.as_str()) {
                let _ = rels.insert(ProjectTree::join_rel(dir_rel, file_name));
            }
        }
    }

    rels.into_iter().collect()
}

fn collect_cargo_roots(
    tree: &ProjectTree,
    route: &RsCodeRoute,
    active_root_dirs: &BTreeSet<String>,
    input_failures: &mut Vec<CodeInputFailureFacts>,
) -> BTreeMap<String, CargoRootFacts> {
    route
        .roots
        .iter()
        .filter(|root| active_root_dirs.contains(&root.root.rel_dir))
        .map(|root| {
            let rel_dir = root.root.rel_dir.clone();
            let rel_path = root.root.cargo_rel_path.clone();
            if tree.file_exists(&rel_path) && tree.file_content(&rel_path).is_none() {
                input_failures.push(CodeInputFailureFacts {
                    rel_path: rel_path.clone(),
                    message: "Failed to read Cargo.toml for code-family root discovery."
                        .to_owned(),
                });
            }
            let parsed = tree
                .file_content(&rel_path)
                .map(|content| toml::from_str::<toml::Value>(content));
            let facts = match parsed {
                Some(Ok(parsed)) => CargoRootFacts {
                    rel_dir: rel_dir.clone(),
                    cargo_rel_path: rel_path.clone(),
                    has_workspace: parsed.get("workspace").is_some(),
                    has_package: parsed.get("package").is_some(),
                    workspace_members: parse_workspace_members(tree, &rel_dir, &parsed),
                },
                Some(Err(parse_error)) => {
                    input_failures.push(CodeInputFailureFacts {
                        rel_path: rel_path.clone(),
                        message: format!("Failed to parse Cargo.toml for code-family root discovery: {parse_error}"),
                    });
                    CargoRootFacts {
                        rel_dir: rel_dir.clone(),
                        cargo_rel_path: rel_path.clone(),
                        has_workspace: false,
                        has_package: false,
                        workspace_members: Vec::new(),
                    }
                }
                None => CargoRootFacts {
                    rel_dir: rel_dir.clone(),
                    cargo_rel_path: rel_path.clone(),
                    has_workspace: false,
                    has_package: false,
                    workspace_members: Vec::new(),
                },
            };
            (rel_dir, facts)
        })
        .collect()
}

fn active_root_dirs(route: &RsCodeRoute) -> BTreeSet<String> {
    let routed_roots = route
        .roots
        .iter()
        .map(|root| root.root.clone())
        .collect::<Vec<_>>();

    match route.scoped_files.as_ref() {
        None => routed_roots
            .into_iter()
            .map(|root| root.rel_dir)
            .collect::<BTreeSet<_>>(),
        Some(scoped_files) => scoped_files
            .iter()
            .filter_map(|rel_path| owning_routed_root(rel_path, &routed_roots))
            .map(|root| root.rel_dir.clone())
            .collect(),
    }
}

fn owning_routed_root<'a>(
    rel_path: &str,
    routed_roots: &'a [guardrail3_app_rs_family_mapper::RsRootView],
) -> Option<&'a guardrail3_app_rs_family_mapper::RsRootView> {
    routed_roots
        .iter()
        .filter(|root| path_belongs_to_root(rel_path, root))
        .max_by_key(|root| root.rel_dir.len())
}

fn path_belongs_to_root(
    rel_path: &str,
    root: &guardrail3_app_rs_family_mapper::RsRootView,
) -> bool {
    rel_path == root.cargo_rel_path
        || rel_path == root.rel_dir
        || root.rel_dir.is_empty()
        || rel_path
            .strip_prefix(root.rel_dir.as_str())
            .is_some_and(|suffix| suffix.starts_with('/'))
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

fn read_policy_map(
    tree: &ProjectTree,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    input_failures: &mut Vec<CodeInputFailureFacts>,
) -> BTreeMap<String, PolicySettings> {
    let mut map = BTreeMap::new();
    let parsed = match tree.file_content("guardrail3.toml") {
        Some(content) => match toml::from_str::<GuardrailConfig>(content) {
            Ok(_) => match toml::from_str::<toml::Value>(content) {
                Ok(parsed) => Some(parsed),
                Err(parse_error) => {
                    input_failures.push(CodeInputFailureFacts {
                        rel_path: "guardrail3.toml".to_owned(),
                        message: format!("Failed to parse guardrail3.toml for code-family policy resolution: {parse_error}"),
                    });
                    None
                }
            },
            Err(parse_error) => {
                input_failures.push(CodeInputFailureFacts {
                    rel_path: "guardrail3.toml".to_owned(),
                    message: format!("Failed to parse guardrail3.toml for code-family policy resolution: {parse_error}"),
                });
                None
            }
        },
        None if tree.file_exists("guardrail3.toml") => {
            input_failures.push(CodeInputFailureFacts {
                rel_path: "guardrail3.toml".to_owned(),
                message: "Failed to read guardrail3.toml for code-family policy resolution."
                    .to_owned(),
            });
            None
        }
        None => None,
    };
    let default_profile = parsed
        .as_ref()
        .and_then(|parsed| parsed.get("profile"))
        .and_then(|value| value.get("name"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned);
    let _ = map.insert(
        String::new(),
        PolicySettings {
            profile_name: default_profile.clone(),
        },
    );

    let Some(parsed) = parsed.as_ref() else {
        return map;
    };
    let rust = parsed.get("rust");

    let resolved_app_paths = resolve_app_paths(cargo_roots);
    let mut configured_app_roots = BTreeSet::new();
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
            if let Some(rel_dir) = resolved_app_paths.get(app_name) {
                let _ = configured_app_roots.insert(rel_dir.clone());
                let _ = map.insert(
                    rel_dir.clone(),
                    PolicySettings {
                        profile_name: profile_name.clone(),
                    },
                );
            }
        }
    }

    let package_roots: BTreeSet<_> = cargo_roots
        .values()
        .filter(|facts| {
            facts.has_package
                && !configured_app_roots
                    .iter()
                    .any(|app_root| rel_is_same_or_descendant(&facts.rel_dir, app_root))
        })
        .map(|facts| facts.rel_dir.clone())
        .collect();

    if let Some(packages) = rust.and_then(|value| value.get("packages")) {
        let profile_name = packages
            .get("type")
            .or_else(|| packages.get("profile"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
            .or_else(|| Some("library".to_owned()))
            .or_else(|| default_profile.clone());
        for rel_dir in &package_roots {
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
    guardrail3_app_core::discover::resolve_app_paths_from_member_dirs(
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
        return policy_map
            .get("")
            .cloned()
            .unwrap_or(PolicySettings { profile_name: None });
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

    policy_map
        .get("")
        .cloned()
        .unwrap_or(PolicySettings { profile_name: None })
}

fn file_parent_rel(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
}

fn rel_is_same_or_descendant(rel: &str, ancestor: &str) -> bool {
    rel == ancestor
        || (!ancestor.is_empty()
            && rel
                .strip_prefix(ancestor)
                .is_some_and(|suffix| suffix.starts_with('/')))
}

fn owning_root_dir<'a>(rel_path: &str, root_dirs: &'a [String]) -> Option<&'a str> {
    let parent = file_parent_rel(rel_path);
    root_dirs
        .iter()
        .filter(|root| {
            root.is_empty()
                || parent == root.as_str()
                || parent
                    .strip_prefix(root.as_str())
                    .is_some_and(|rest| rest.starts_with('/'))
        })
        .max_by_key(|root| root.len())
        .map(String::as_str)
}
