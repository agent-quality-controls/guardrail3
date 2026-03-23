use std::collections::{BTreeMap, BTreeSet};

use crate::app::rs::checks::hooks::shell::parse_script;
use crate::domain::config::types::GuardrailConfig;
use crate::domain::project_tree::ProjectTree;
use crate::ports::outbound::ToolChecker;

use super::discover::{
    is_integration_test_path, is_src_path, is_test_sidecar_path, rust_file_rels,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestRootKind {
    WorkspaceRoot,
    StandalonePackageRoot,
}

impl TestRootKind {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceRoot => "workspace root",
            Self::StandalonePackageRoot => "standalone package root",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToolFacts {
    pub installed: bool,
}

#[derive(Debug, Clone)]
pub struct HookFacts {
    pub matching_files: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TestRootFacts {
    pub rel_dir: String,
    pub kind: TestRootKind,
    pub cargo_rel_path: String,
    pub mutants_rel_path: String,
    pub mutants_exists: bool,
    pub mutants_parsed: Option<toml::Value>,
    pub nextest_rel_path: String,
    pub nextest_exists: bool,
    pub nextest_parsed: Option<toml::Value>,
    pub nextest_parse_error: Option<String>,
    pub tokio_present: bool,
    pub has_mutants_profile: bool,
}

#[derive(Debug, Clone)]
pub struct TestFileFacts {
    pub rel_path: String,
    pub root_rel_dir: String,
    pub is_src_file: bool,
    pub is_integration_test_file: bool,
    pub is_test_sidecar_file: bool,
}

#[derive(Debug, Clone)]
pub struct TestCoverageFacts {
    pub root_rel_dir: String,
    pub has_any_tests: bool,
    pub public_fn_count: usize,
    pub test_fn_count: usize,
    pub integration_test_exists: bool,
}

impl TestCoverageFacts {
    pub fn new(root_rel_dir: String) -> Self {
        Self {
            root_rel_dir,
            has_any_tests: false,
            public_fn_count: 0,
            test_fn_count: 0,
            integration_test_exists: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputFailureFacts {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct TestFacts {
    pub tool: ToolFacts,
    pub hook: HookFacts,
    pub roots: Vec<TestRootFacts>,
    pub files: Vec<TestFileFacts>,
    pub input_failures: Vec<InputFailureFacts>,
}

#[derive(Debug, Clone)]
struct ParsedGuardrail {
    parse_error: Option<String>,
}

#[derive(Debug, Clone)]
struct CargoRootFacts {
    rel_dir: String,
    cargo_rel_path: String,
    parsed: Option<toml::Value>,
    has_workspace: bool,
    has_package: bool,
    workspace_members: Vec<String>,
}

pub fn collect(tree: &ProjectTree, tc: &dyn ToolChecker) -> TestFacts {
    let mut input_failures = Vec::new();
    let guardrail = parse_guardrail(tree);
    if let Some(parse_error) = guardrail
        .as_ref()
        .and_then(|value| value.parse_error.as_ref())
    {
        input_failures.push(InputFailureFacts {
            rel_path: "guardrail3.toml".to_owned(),
            message: parse_error.clone(),
        });
    }

    let cargo_roots = collect_cargo_roots(tree, &mut input_failures);
    let workspace_roots: BTreeSet<_> = cargo_roots
        .values()
        .filter(|root| root.has_workspace)
        .map(|root| root.rel_dir.clone())
        .collect();
    let workspace_members: BTreeSet<_> = cargo_roots
        .values()
        .flat_map(|root| root.workspace_members.iter().cloned())
        .collect();
    let standalone_package_roots: BTreeSet<_> = cargo_roots
        .values()
        .filter(|root| root.has_package && !workspace_members.contains(&root.rel_dir))
        .map(|root| root.rel_dir.clone())
        .collect();

    let mut roots = Vec::new();
    for rel_dir in &workspace_roots {
        roots.push(build_root_facts(
            tree,
            rel_dir,
            TestRootKind::WorkspaceRoot,
            &cargo_roots,
            guardrail.as_ref(),
            &mut input_failures,
        ));
    }
    for rel_dir in &standalone_package_roots {
        roots.push(build_root_facts(
            tree,
            rel_dir,
            TestRootKind::StandalonePackageRoot,
            &cargo_roots,
            guardrail.as_ref(),
            &mut input_failures,
        ));
    }
    roots.sort_by(|a, b| a.rel_dir.cmp(&b.rel_dir));

    let root_dirs: Vec<String> = roots.iter().map(|root| root.rel_dir.clone()).collect();
    let files = rust_file_rels(tree)
        .into_iter()
        .filter_map(|rel_path| {
            let root_rel_dir = nearest_root_dir(&rel_path, &root_dirs)?;
            Some(TestFileFacts {
                is_src_file: is_src_path(&rel_path, &root_rel_dir),
                is_integration_test_file: is_integration_test_path(&rel_path, &root_rel_dir),
                is_test_sidecar_file: is_test_sidecar_path(&rel_path, &root_rel_dir),
                rel_path,
                root_rel_dir,
            })
        })
        .collect();

    TestFacts {
        tool: ToolFacts {
            installed: tc.is_installed("cargo-mutants"),
        },
        hook: HookFacts {
            matching_files: collect_mutation_hook_files(tree),
        },
        roots,
        files,
        input_failures,
    }
}

fn parse_guardrail(tree: &ProjectTree) -> Option<ParsedGuardrail> {
    let content = tree.file_content("guardrail3.toml")?;
    match toml::from_str::<GuardrailConfig>(content) {
        Ok(_) => Some(ParsedGuardrail { parse_error: None }),
        Err(parse_error) => Some(ParsedGuardrail {
            parse_error: Some(format!(
                "Failed to parse guardrail3.toml for test-family policy resolution: {parse_error}"
            )),
        }),
    }
}

fn collect_cargo_roots(
    tree: &ProjectTree,
    input_failures: &mut Vec<InputFailureFacts>,
) -> BTreeMap<String, CargoRootFacts> {
    let mut dirs = BTreeSet::new();
    if tree.file_exists("Cargo.toml") {
        let _ = dirs.insert(String::new());
    }
    dirs.extend(tree.dirs_with_file("Cargo.toml"));

    dirs.into_iter()
        .map(|rel_dir| {
            let cargo_rel_path = if rel_dir.is_empty() {
                "Cargo.toml".to_owned()
            } else {
                ProjectTree::join_rel(&rel_dir, "Cargo.toml")
            };
            let parsed = match tree.file_content(&cargo_rel_path) {
                Some(content) => match toml::from_str::<toml::Value>(content) {
                    Ok(parsed) => Some(parsed),
                    Err(parse_error) => {
                        input_failures.push(InputFailureFacts {
                            rel_path: cargo_rel_path.clone(),
                            message: format!(
                                "Failed to parse Cargo.toml for test-family root discovery: {parse_error}"
                            ),
                        });
                        None
                    }
                },
                None => None,
            };
            let facts = CargoRootFacts {
                rel_dir: rel_dir.clone(),
                cargo_rel_path,
                has_workspace: parsed.as_ref().is_some_and(|parsed| parsed.get("workspace").is_some()),
                has_package: parsed.as_ref().is_some_and(|parsed| parsed.get("package").is_some()),
                workspace_members: parsed
                    .as_ref()
                    .map(|parsed| parse_workspace_members(tree, &rel_dir, parsed))
                    .unwrap_or_default(),
                parsed,
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

fn build_root_facts(
    tree: &ProjectTree,
    rel_dir: &str,
    kind: TestRootKind,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    _guardrail: Option<&ParsedGuardrail>,
    input_failures: &mut Vec<InputFailureFacts>,
) -> TestRootFacts {
    let cargo = cargo_roots
        .get(rel_dir)
        .expect("expected discovered cargo root");
    let mutants_rel_path = join_under_root(rel_dir, ".cargo/mutants.toml");
    let nextest_rel_path = join_under_root(rel_dir, ".config/nextest.toml");
    let (mutants_exists, mutants_parsed, _) =
        parse_optional_toml(tree, &mutants_rel_path, "mutants config", input_failures);
    let (nextest_exists, nextest_parsed, nextest_parse_error) =
        parse_optional_toml(tree, &nextest_rel_path, "nextest config", input_failures);

    TestRootFacts {
        rel_dir: rel_dir.to_owned(),
        kind,
        cargo_rel_path: cargo.cargo_rel_path.clone(),
        mutants_rel_path,
        mutants_exists,
        mutants_parsed,
        nextest_rel_path,
        nextest_exists,
        nextest_parsed,
        nextest_parse_error,
        tokio_present: root_has_tokio(tree, cargo, cargo_roots),
        has_mutants_profile: cargo
            .parsed
            .as_ref()
            .and_then(|parsed| parsed.get("profile"))
            .and_then(|profile| profile.get("mutants"))
            .is_some(),
    }
}

fn parse_optional_toml(
    tree: &ProjectTree,
    rel_path: &str,
    label: &str,
    input_failures: &mut Vec<InputFailureFacts>,
) -> (bool, Option<toml::Value>, Option<String>) {
    let Some(content) = tree.file_content(rel_path) else {
        return (false, None, None);
    };
    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => (true, Some(parsed), None),
        Err(parse_error) => {
            let message = format!("Failed to parse {label}: {parse_error}");
            input_failures.push(InputFailureFacts {
                rel_path: rel_path.to_owned(),
                message: message.clone(),
            });
            (true, None, Some(message))
        }
    }
}

fn root_has_tokio(
    tree: &ProjectTree,
    cargo: &CargoRootFacts,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
) -> bool {
    let Some(parsed) = cargo.parsed.as_ref() else {
        return false;
    };
    let workspace_deps = parsed
        .get("workspace")
        .and_then(|workspace| workspace.get("dependencies"))
        .and_then(toml::Value::as_table);

    if cargo_toml_has_tokio(parsed, workspace_deps) {
        return true;
    }

    if cargo.has_workspace {
        for member_dir in &cargo.workspace_members {
            let member_cargo = if member_dir.is_empty() {
                "Cargo.toml".to_owned()
            } else {
                ProjectTree::join_rel(member_dir, "Cargo.toml")
            };
            let Some(content) = tree.file_content(&member_cargo) else {
                continue;
            };
            let Ok(parsed_member) = toml::from_str::<toml::Value>(content) else {
                continue;
            };
            if cargo_toml_has_tokio(&parsed_member, workspace_deps) {
                return true;
            }
        }
    } else if cargo.has_package {
        let _ = cargo_roots;
    }

    false
}

fn cargo_toml_has_tokio(
    parsed: &toml::Value,
    workspace_deps: Option<&toml::map::Map<String, toml::Value>>,
) -> bool {
    ["dependencies", "build-dependencies", "dev-dependencies"]
        .iter()
        .any(|section| {
            parsed
                .get(*section)
                .and_then(toml::Value::as_table)
                .is_some_and(|table| {
                    table
                        .iter()
                        .any(|(dep_name, spec)| dependency_is_tokio(dep_name, spec, workspace_deps))
                })
        })
}

fn dependency_is_tokio(
    dep_name: &str,
    spec: &toml::Value,
    workspace_deps: Option<&toml::map::Map<String, toml::Value>>,
) -> bool {
    if dep_name == "tokio" {
        return true;
    }
    match spec {
        toml::Value::String(_) => false,
        toml::Value::Table(table) => {
            if table.get("package").and_then(toml::Value::as_str) == Some("tokio") {
                return true;
            }
            if table.get("workspace").and_then(toml::Value::as_bool) == Some(true) {
                let Some(workspace_deps) = workspace_deps else {
                    return dep_name == "tokio";
                };
                let Some(workspace_spec) = workspace_deps.get(dep_name) else {
                    return dep_name == "tokio";
                };
                return dependency_is_tokio(dep_name, workspace_spec, None);
            }
            false
        }
        _ => false,
    }
}

fn nearest_root_dir(rel_path: &str, root_dirs: &[String]) -> Option<String> {
    root_dirs
        .iter()
        .filter(|root_rel| {
            root_rel.is_empty()
                || rel_path == *root_rel
                || rel_path
                    .strip_prefix(root_rel.as_str())
                    .is_some_and(|rest| rest.starts_with('/'))
        })
        .max_by_key(|root_rel| root_rel.len())
        .cloned()
}

fn join_under_root(root_rel_dir: &str, child_rel: &str) -> String {
    if root_rel_dir.is_empty() {
        child_rel.to_owned()
    } else {
        ProjectTree::join_rel(root_rel_dir, child_rel)
    }
}

fn collect_mutation_hook_files(tree: &ProjectTree) -> Vec<String> {
    let mut files = Vec::new();
    for rel_path in tree.content.keys() {
        if rel_path.ends_with("pre-commit") {
            let Some(content) = tree.file_content(rel_path) else {
                continue;
            };
            if parse_script(content)
                .executable_lines
                .iter()
                .any(executable_line_has_mutation_hook)
            {
                files.push(rel_path.clone());
            }
        }
    }
    files.sort();
    files
}

fn executable_line_has_mutation_hook(
    line: &crate::app::rs::checks::hooks::shell::ExecutableLine<'_>,
) -> bool {
    line.command_text.contains("cargo mutants") || line.command_text.contains("cargo-mutants")
}
