use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsRootView;
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
use guardrail3_outbound_traits::ToolChecker;

mod components;
mod mutation;

use self::mutation::collect_mutation_hook_state;
use super::facts::{InputFailureFacts, TestFacts, TestRootFacts};

#[derive(Debug, Clone)]
struct CargoRootFacts {
    cargo_rel_path: String,
    parsed: Option<toml::Value>,
    has_workspace: bool,
    has_package: bool,
    workspace_members: Vec<String>,
}

pub fn collect(tree: &ProjectTree, routed_roots: &[RsRootView], tc: &dyn ToolChecker) -> TestFacts {
    let mut input_failures = Vec::new();
    let cargo_roots = collect_cargo_roots(tree, routed_roots, &mut input_failures);
    let mut roots = collect_test_root_dirs(&cargo_roots)
        .into_iter()
        .map(|rel_dir| build_root_facts(tree, &rel_dir, &cargo_roots, &mut input_failures))
        .collect::<Vec<_>>();
    roots.sort_by(|a, b| a.rel_dir.cmp(&b.rel_dir));

    let root_dirs: Vec<String> = roots.iter().map(|root| root.rel_dir.clone()).collect();
    let local_package_names = cargo_roots
        .values()
        .filter_map(|root| {
            root.parsed
                .as_ref()
                .and_then(components::manifest_package_name)
        })
        .collect();
    let files = rust_file_rels(tree)
        .into_iter()
        .filter_map(|rel_path| {
            let root_rel_dir = owning_root_dir(&rel_path, &roots, &root_dirs)?;
            let root = roots
                .iter()
                .find(|candidate| candidate.rel_dir == root_rel_dir)?;
            Some(components::classify_file(root, &rel_path))
        })
        .collect();

    TestFacts {
        cargo_mutants_installed: tc.is_installed("cargo-mutants"),
        local_package_names,
        roots,
        files,
        input_failures,
    }
}

pub fn rust_file_rels(tree: &ProjectTree) -> Vec<String> {
    let mut rels: Vec<String> = tree
        .structure()
        .iter()
        .flat_map(|(dir_rel, entry)| {
            entry.files().iter().filter_map(|file_name| {
                if !file_name.ends_with(".rs") {
                    return None;
                }
                let rel = ProjectTree::join_rel(dir_rel, file_name);
                if is_fixture_path(&rel) || is_generated_path(&rel) {
                    None
                } else {
                    Some(rel)
                }
            })
        })
        .collect();
    rels.sort();
    rels
}

pub fn is_fixture_path(rel_path: &str) -> bool {
    rel_path.contains("/tests/fixtures/")
        || rel_path.starts_with("tests/fixtures/")
        || rel_path.contains("_tests/fixtures/")
        || rel_path.contains("assertions/src/fixtures/")
        || rel_path.contains("test_support/src/fixtures/")
}

fn is_generated_path(rel_path: &str) -> bool {
    rel_path == "target" || rel_path.starts_with("target/") || rel_path.contains("/target/")
}

pub fn root_relative<'a>(rel_path: &'a str, root_rel_dir: &str) -> &'a str {
    if root_rel_dir.is_empty() {
        rel_path
    } else {
        rel_path
            .strip_prefix(root_rel_dir)
            .and_then(|rest| rest.strip_prefix('/'))
            .unwrap_or(rel_path)
    }
}

pub fn path_is_under(rel_path: &str, prefix: &str) -> bool {
    rel_path == prefix
        || rel_path
            .strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with('/'))
}

pub fn file_stem(rel_path: &str) -> Option<&str> {
    rel_path
        .rsplit('/')
        .next()
        .and_then(|name| name.strip_suffix(".rs"))
}

pub fn parent_dir(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
}

pub fn join_under_root(root_rel_dir: &str, child_rel: &str) -> String {
    if root_rel_dir.is_empty() {
        child_rel.to_owned()
    } else {
        ProjectTree::join_rel(root_rel_dir, child_rel)
    }
}

fn collect_cargo_roots(
    tree: &ProjectTree,
    routed_roots: &[RsRootView],
    input_failures: &mut Vec<InputFailureFacts>,
) -> BTreeMap<String, CargoRootFacts> {
    routed_roots
        .iter()
        .map(|root| {
            let rel_dir = root.rel_dir().to_owned();
            let cargo_rel_path = root.cargo_rel_path().to_owned();
            let parsed = match read_cached_or_fs(tree, &cargo_rel_path) {
                Ok(Some(content)) => match toml::from_str::<toml::Value>(&content) {
                    Ok(parsed) => Some(parsed),
                    Err(parse_error) => {
                        input_failures.push(InputFailureFacts {
                            root_rel_dir: rel_dir.clone(),
                            rel_path: cargo_rel_path.clone(),
                            message: format!(
                                "Failed to parse Cargo.toml for test-family root discovery: {parse_error}"
                            ),
                        });
                        None
                    }
                },
                Ok(None) => None,
                Err(read_error) => {
                    input_failures.push(InputFailureFacts {
                        root_rel_dir: rel_dir.clone(),
                        rel_path: cargo_rel_path.clone(),
                        message: format!(
                            "Failed to read Cargo.toml for test-family root discovery: {read_error}"
                        ),
                    });
                    None
                }
            };
            let facts = CargoRootFacts {
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

fn collect_test_root_dirs(cargo_roots: &BTreeMap<String, CargoRootFacts>) -> Vec<String> {
    let mut root_dirs = BTreeSet::new();
    for rel_dir in cargo_roots.keys() {
        let _ = root_dirs.insert(
            component_container_root(rel_dir, cargo_roots).unwrap_or_else(|| rel_dir.clone()),
        );
    }
    root_dirs.into_iter().collect()
}

fn component_container_root(
    rel_dir: &str,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
) -> Option<String> {
    let candidate = if matches!(
        rel_dir,
        "crates/runtime"
            | "crates/assertions"
            | "crates/assertions_common"
            | "assertions"
            | "crates/test_support"
            | "test_support"
    ) {
        Some(String::new())
    } else {
        rel_dir
            .strip_suffix("/crates/runtime")
            .or_else(|| rel_dir.strip_suffix("/crates/assertions"))
            .or_else(|| rel_dir.strip_suffix("/crates/assertions_common"))
            .or_else(|| rel_dir.strip_suffix("/assertions"))
            .or_else(|| rel_dir.strip_suffix("/crates/test_support"))
            .or_else(|| rel_dir.strip_suffix("/test_support"))
            .map(ToOwned::to_owned)
    };

    let candidate = candidate?;
    let candidate_has_package = cargo_roots
        .get(&candidate)
        .is_some_and(|root| root.has_package);
    let candidate_has_runtime =
        cargo_roots.contains_key(&join_under_root(&candidate, "crates/runtime"));
    (candidate_has_package || candidate_has_runtime).then_some(candidate)
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
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    input_failures: &mut Vec<InputFailureFacts>,
) -> TestRootFacts {
    let cargo = root_cargo_facts(rel_dir, cargo_roots);
    let mutants_rel_path = join_under_root(rel_dir, ".cargo/mutants.toml");
    let nextest_rel_path = join_under_root(rel_dir, ".config/nextest.toml");
    let (mutants_exists, mutants_parsed, _) = parse_optional_toml(
        tree,
        rel_dir,
        &mutants_rel_path,
        "mutants config",
        input_failures,
    );
    let (nextest_exists, nextest_parsed, _nextest_parse_error) = parse_optional_toml(
        tree,
        rel_dir,
        &nextest_rel_path,
        "nextest config",
        input_failures,
    );
    let hook_state = collect_mutation_hook_state(
        tree,
        rel_dir,
        &active_hook_root_dirs(rel_dir, cargo_roots),
        input_failures,
    );

    TestRootFacts {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path: cargo
            .map(|facts| facts.cargo_rel_path.clone())
            .unwrap_or_else(|| join_under_root(rel_dir, "crates/runtime/Cargo.toml")),
        mutants_rel_path,
        mutants_exists,
        mutants_parsed,
        nextest_rel_path,
        nextest_exists,
        nextest_parsed,
        tokio_dependency_present: cargo
            .is_some_and(|facts| root_has_tokio(tree, facts, cargo_roots)),
        has_mutants_profile: cargo
            .and_then(|facts| facts.parsed.as_ref())
            .and_then(|parsed| parsed.get("profile"))
            .and_then(|profile| profile.get("mutants"))
            .is_some(),
        mutation_hook_active: hook_state.active,
        mutation_hook_files: hook_state.files,
        components: components::collect_components(
            tree,
            rel_dir,
            cargo.is_some_and(|facts| facts.has_package),
            input_failures,
        ),
    }
}

fn root_cargo_facts<'a>(
    rel_dir: &str,
    cargo_roots: &'a BTreeMap<String, CargoRootFacts>,
) -> Option<&'a CargoRootFacts> {
    cargo_roots.get(rel_dir).or_else(|| {
        let runtime_rel_dir = join_under_root(rel_dir, "crates/runtime");
        cargo_roots.get(&runtime_rel_dir)
    })
}

fn active_hook_root_dirs(
    rel_dir: &str,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
) -> Vec<String> {
    let mut roots = BTreeSet::new();
    if rel_dir.is_empty() {
        let _ = roots.insert(String::new());
    }
    for (workspace_rel, facts) in cargo_roots {
        if !facts.has_workspace {
            continue;
        }
        if workspace_rel == rel_dir
            || facts
                .workspace_members
                .iter()
                .any(|member| member == rel_dir)
        {
            let _ = roots.insert(workspace_rel.clone());
        }
    }
    roots.into_iter().collect()
}

fn parse_optional_toml(
    tree: &ProjectTree,
    root_rel_dir: &str,
    rel_path: &str,
    label: &str,
    input_failures: &mut Vec<InputFailureFacts>,
) -> (bool, Option<toml::Value>, Option<String>) {
    let content = match read_cached_or_fs(tree, rel_path) {
        Ok(Some(content)) => content,
        Ok(None) => return (false, None, None),
        Err(read_error) => {
            let message = format!("Failed to read {label}: {read_error}");
            input_failures.push(InputFailureFacts {
                root_rel_dir: root_rel_dir.to_owned(),
                rel_path: rel_path.to_owned(),
                message: message.clone(),
            });
            return (true, None, Some(message));
        }
    };
    match toml::from_str::<toml::Value>(&content) {
        Ok(parsed) => (true, Some(parsed), None),
        Err(parse_error) => {
            let message = format!("Failed to parse {label}: {parse_error}");
            input_failures.push(InputFailureFacts {
                root_rel_dir: root_rel_dir.to_owned(),
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

fn owning_root_dir(
    rel_path: &str,
    roots: &[TestRootFacts],
    root_dirs: &[String],
) -> Option<String> {
    roots
        .iter()
        .filter(|root| root_component_owns_file(root, rel_path))
        .map(|root| root.rel_dir.clone())
        .max_by_key(|root_rel| root_rel.len())
        .or_else(|| nearest_root_dir(rel_path, root_dirs))
}

fn root_component_owns_file(root: &TestRootFacts, rel_path: &str) -> bool {
    if root_has_test_support_file(root, rel_path) {
        return true;
    }

    root.components.iter().any(|component| {
        let runtime_src = ProjectTree::join_rel(&component.runtime_rel_dir, "src");
        let assertions_src = ProjectTree::join_rel(&component.assertions_rel_dir, "src");

        path_is_under(rel_path, &runtime_src)
            || component
                .external_harnesses
                .iter()
                .any(|harness| harness == rel_path)
            || path_is_under(rel_path, &assertions_src)
    })
}

fn root_has_test_support_file(root: &TestRootFacts, rel_path: &str) -> bool {
    [
        join_under_root(&root.rel_dir, "test_support/src"),
        join_under_root(&root.rel_dir, "crates/test_support/src"),
    ]
    .into_iter()
    .any(|test_support_src| {
        rel_path == test_support_src || path_is_under(rel_path, &test_support_src)
    })
}

fn read_cached_or_fs(tree: &ProjectTree, rel_path: &str) -> Result<Option<String>, std::io::Error> {
    if let Some(content) = tree.file_content(rel_path) {
        return Ok(Some(content.to_owned()));
    }
    if !tree.file_exists(rel_path) {
        return Ok(None);
    }
    match tree.abs_path(rel_path) {
        Some(abs) => guardrail3_shared_fs::read_file_err(&abs).map(Some),
        None => Ok(None),
    }
}
