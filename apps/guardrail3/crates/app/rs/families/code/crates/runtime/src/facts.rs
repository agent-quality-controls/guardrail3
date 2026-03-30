mod comments;
mod policy;

use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsCodeRoute;
use guardrail3_domain_project_tree::ProjectTree;

use super::discover::{is_test_root_path, rust_file_rels};

#[derive(Debug, Clone)]
pub struct RustCodeFileFacts {
    pub(crate) rel_path: String,
    pub(crate) is_test_root: bool,
    pub(crate) profile_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UnsafeCodeLintFacts {
    pub(crate) cargo_rel_path: String,
    pub(crate) lint_level: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CodeFacts {
    pub(crate) files: Vec<RustCodeFileFacts>,
    pub(crate) structural_caps: Vec<StructuralCapFacts>,
    pub(crate) unsafe_code_lints: Vec<UnsafeCodeLintFacts>,
    pub(crate) exception_comments: Vec<ExceptionCommentFacts>,
    pub(crate) input_failures: Vec<CodeInputFailureFacts>,
}

#[derive(Debug, Clone)]
pub struct StructuralCapFacts {
    pub(crate) root_rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) max_module_depth: usize,
    pub(crate) max_sibling_dirs: usize,
    pub(crate) max_sibling_rs_files: usize,
}

pub fn collect(tree: &ProjectTree, route: &RsCodeRoute) -> CodeFacts {
    let mut input_failures = Vec::new();
    let active_root_dirs = active_root_dirs(route);
    let cargo_roots = collect_cargo_roots(tree, route, &active_root_dirs, &mut input_failures);
    let root_dirs = cargo_roots.keys().cloned().collect::<Vec<_>>();
    let policy_map = policy::read_policy_map(tree, &cargo_roots, &mut input_failures);

    let files = rust_file_rels(tree)
        .into_iter()
        .filter(|rel_path| owning_root_dir(rel_path, &root_dirs).is_some())
        .map(|rel_path| RustCodeFileFacts {
            profile_name: policy::policy_settings_for(file_parent_rel(&rel_path), &policy_map)
                .profile_name,
            is_test_root: is_test_root_path(&rel_path),
            rel_path,
        })
        .collect();
    let structural_caps = cargo_roots
        .values()
        .map(|root| measure_root_structure(tree, &root.rel_dir, &root.cargo_rel_path))
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

    let exception_comments = comments::collect_exception_comments(tree, &root_dirs);

    CodeFacts {
        files,
        structural_caps,
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
    pub(crate) rel_path: String,
    pub(crate) line: usize,
    pub(crate) line_text: String,
}

#[derive(Debug, Clone)]
pub struct CodeInputFailureFacts {
    pub(crate) rel_path: String,
    pub(crate) message: String,
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

fn collect_cargo_roots(
    tree: &ProjectTree,
    route: &RsCodeRoute,
    active_root_dirs: &BTreeSet<String>,
    input_failures: &mut Vec<CodeInputFailureFacts>,
) -> BTreeMap<String, CargoRootFacts> {
    route
        .roots()
        .iter()
        .filter(|root| active_root_dirs.contains(root.root().rel_dir()))
        .map(|root| {
            let rel_dir = root.root().rel_dir().to_owned();
            let rel_path = root.root().cargo_rel_path().to_owned();
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
        .roots()
        .iter()
        .map(|root| root.root().clone())
        .collect::<Vec<_>>();

    match route.scoped_files() {
        None => routed_roots
            .into_iter()
            .map(|root| root.rel_dir().to_owned())
            .collect::<BTreeSet<_>>(),
        Some(scoped_files) => scoped_files
            .iter()
            .filter_map(|rel_path| owning_routed_root(rel_path, &routed_roots))
            .map(|root| root.rel_dir().to_owned())
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
        .max_by_key(|root| root.rel_dir().len())
}

fn path_belongs_to_root(
    rel_path: &str,
    root: &guardrail3_app_rs_family_mapper::RsRootView,
) -> bool {
    rel_path == root.cargo_rel_path()
        || rel_path == root.rel_dir()
        || root.rel_dir().is_empty()
        || rel_path
            .strip_prefix(root.rel_dir())
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

fn measure_root_structure(
    tree: &ProjectTree,
    root_rel_dir: &str,
    cargo_rel_path: &str,
) -> StructuralCapFacts {
    let rust_related_dirs = collect_rust_related_dirs(tree, root_rel_dir);
    let mut max_module_depth = 0usize;
    let mut max_sibling_dirs = 0usize;
    let mut max_sibling_rs_files = 0usize;

    for dir_rel in &rust_related_dirs {
        let Some(entry) = tree.structure().get(dir_rel) else {
            continue;
        };
        let sibling_dirs = entry
            .dirs()
            .iter()
            .filter(|dir_name| {
                rust_related_dirs.contains(&ProjectTree::join_rel(dir_rel, dir_name))
            })
            .count();
        max_sibling_dirs = max_sibling_dirs.max(sibling_dirs);

        let rs_file_count = entry
            .files()
            .iter()
            .filter(|file_name| file_name.ends_with(".rs"))
            .count();
        max_sibling_rs_files = max_sibling_rs_files.max(rs_file_count);
    }

    for rel_path in rust_file_rels(tree).into_iter().filter(|rel_path| {
        owning_root_dir(rel_path, &[root_rel_dir.to_owned()]).is_some()
            && !is_generated_path(rel_path)
    }) {
        max_module_depth = max_module_depth.max(module_depth(root_rel_dir, &rel_path));
    }

    StructuralCapFacts {
        root_rel_dir: root_rel_dir.to_owned(),
        cargo_rel_path: cargo_rel_path.to_owned(),
        max_module_depth,
        max_sibling_dirs,
        max_sibling_rs_files,
    }
}

fn collect_rust_related_dirs(tree: &ProjectTree, root_rel_dir: &str) -> BTreeSet<String> {
    let mut dirs = BTreeSet::new();
    for rel_path in rust_file_rels(tree).into_iter().filter(|rel_path| {
        owning_root_dir(rel_path, &[root_rel_dir.to_owned()]).is_some()
            && !is_generated_path(rel_path)
    }) {
        let mut current = file_parent_rel(&rel_path).to_owned();
        loop {
            let _ = dirs.insert(current.clone());
            if current == root_rel_dir || current.is_empty() {
                break;
            }
            let Some((parent, _)) = current.rsplit_once('/') else {
                current.clear();
                continue;
            };
            current = parent.to_owned();
        }
    }
    dirs
}

fn is_generated_path(rel_path: &str) -> bool {
    rel_path == "target" || rel_path.starts_with("target/") || rel_path.contains("/target/")
}

fn module_depth(root_rel_dir: &str, rel_path: &str) -> usize {
    let root_segments = path_segments(root_rel_dir);
    let path_segments = path_segments(rel_path);
    if path_segments.len() <= root_segments.len() {
        return 0;
    }
    let relative = &path_segments[root_segments.len()..];
    let file_name = *relative.last().unwrap_or(&"");
    let dir_depth = relative.len().saturating_sub(1);
    if matches!(file_name, "lib.rs" | "main.rs" | "mod.rs") {
        dir_depth
    } else {
        dir_depth.saturating_add(1)
    }
}

fn path_segments(rel_path: &str) -> Vec<&str> {
    rel_path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect()
}
