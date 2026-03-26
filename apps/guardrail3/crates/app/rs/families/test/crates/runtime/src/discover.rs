use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_hooks_shared::hook_shell::parse_script;
use guardrail3_app_rs_family_mapper::RsRootView;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_outbound_traits::ToolChecker;

use super::facts::{
    DiscoveredTestFile, InputFailureFacts, SidecarFacts, TestComponentFacts, TestFacts,
    TestFileKind, TestRootFacts,
};

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
    let mut roots = cargo_roots
        .keys()
        .map(|rel_dir| build_root_facts(tree, rel_dir, &cargo_roots, &mut input_failures))
        .collect::<Vec<_>>();
    roots.sort_by(|a, b| a.rel_dir.cmp(&b.rel_dir));

    let root_dirs: Vec<String> = roots.iter().map(|root| root.rel_dir.clone()).collect();
    let local_package_names = cargo_roots
        .values()
        .filter_map(|root| root.parsed.as_ref().and_then(manifest_package_name))
        .collect();
    let files = rust_file_rels(tree)
        .into_iter()
        .filter_map(|rel_path| {
            let root_rel_dir = owning_root_dir(&rel_path, &roots, &root_dirs)?;
            let root = roots
                .iter()
                .find(|candidate| candidate.rel_dir == root_rel_dir)?;
            Some(classify_file(root, &rel_path))
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
        .structure
        .iter()
        .flat_map(|(dir_rel, entry)| {
            entry.files.iter().filter_map(|file_name| {
                if !file_name.ends_with(".rs") {
                    return None;
                }
                let rel = ProjectTree::join_rel(dir_rel, file_name);
                if is_fixture_path(&rel) {
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
    rel_path.contains("/tests/fixtures/") || rel_path.starts_with("tests/fixtures/")
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
            let rel_dir = root.rel_dir.clone();
            let cargo_rel_path = root.cargo_rel_path.clone();
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
    let cargo = cargo_roots
        .get(rel_dir)
        .expect("expected discovered cargo root");
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

    TestRootFacts {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path: cargo.cargo_rel_path.clone(),
        mutants_rel_path,
        mutants_exists,
        mutants_parsed,
        nextest_rel_path,
        nextest_exists,
        nextest_parsed,
        tokio_dependency_present: root_has_tokio(tree, cargo, cargo_roots),
        has_mutants_profile: cargo
            .parsed
            .as_ref()
            .and_then(|parsed| parsed.get("profile"))
            .and_then(|profile| profile.get("mutants"))
            .is_some(),
        mutation_hook_files: collect_mutation_hook_files(tree, rel_dir),
        components: collect_components(tree, rel_dir, input_failures),
    }
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
    let test_support_src = join_under_root(&root.rel_dir, "test_support/src");
    rel_path == test_support_src || path_is_under(rel_path, &test_support_src)
}

fn collect_mutation_hook_files(tree: &ProjectTree, root_rel_dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    for rel_path in [
        join_under_root(root_rel_dir, ".githooks/pre-commit"),
        join_under_root(root_rel_dir, "hooks/pre-commit"),
    ] {
        if let Some(content) = tree.file_content(&rel_path) {
            if parse_script(content)
                .executable_lines
                .iter()
                .any(executable_line_has_mutation_hook)
            {
                files.push(rel_path.to_owned());
            }
        }
    }
    let hook_dir_rel = join_under_root(root_rel_dir, ".githooks/pre-commit.d");
    if let Some(dir) = tree.dir_contents(&hook_dir_rel) {
        for file_name in &dir.files {
            let rel_path = ProjectTree::join_rel(&hook_dir_rel, file_name);
            let Ok(content) = guardrail3_shared_fs::read_file_err(&tree.abs_path(&rel_path)) else {
                continue;
            };
            if parse_script(&content)
                .executable_lines
                .iter()
                .any(executable_line_has_mutation_hook)
            {
                files.push(rel_path);
            }
        }
    }
    files.sort();
    files
}

fn executable_line_has_mutation_hook(
    line: &guardrail3_app_rs_family_hooks_shared::hook_shell::ExecutableLine<'_>,
) -> bool {
    is_cargo_mutants_command(line.command_text)
}

fn is_cargo_mutants_command(command_text: &str) -> bool {
    let tokens = shell_words(command_text);
    let mut parts = tokens.iter().map(String::as_str).peekable();

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    let Some(first) = parts.next() else {
        return false;
    };

    let first = normalize_command_token(first);
    if first == "env" {
        while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
            let _ = parts.next();
        }
        while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
            let _ = parts.next();
        }
        let Some(next) = parts.next() else {
            return false;
        };
        return match normalize_command_token(next) {
            "cargo" => is_cargo_mutants_invocation(&mut parts),
            "cargo-mutants" => !parts.any(is_help_or_version_flag),
            _ => false,
        };
    }

    match first {
        "cargo" => is_cargo_mutants_invocation(&mut parts),
        "cargo-mutants" => !parts.any(is_help_or_version_flag),
        _ => false,
    }
}

fn is_cargo_mutants_invocation<'a, I>(parts: &mut std::iter::Peekable<I>) -> bool
where
    I: Iterator<Item = &'a str>,
{
    if matches!(parts.peek(), Some(token) if token.starts_with('+')) {
        let _ = parts.next();
    }

    while let Some(token) = parts.peek().copied() {
        if !token.starts_with('-') {
            break;
        }

        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if cargo_global_flag_takes_value(flag) {
            let _ = parts.next();
        }
    }

    parts.next() == Some("mutants") && !parts.any(is_help_or_version_flag)
}

fn cargo_global_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "--config" | "-Z" | "--manifest-path" | "--color" | "--target" | "--target-dir" | "--jobs"
    )
}

fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

fn normalize_command_token(token: &str) -> &str {
    token.rsplit('/').next().unwrap_or(token)
}

fn looks_like_env_assignment(token: &str) -> bool {
    let Some((name, _value)) = token.split_once('=') else {
        return false;
    };
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn shell_words(command_text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut chars = command_text.chars().peekable();
    let mut single_quoted = false;
    let mut double_quoted = false;

    while let Some(ch) = chars.next() {
        match ch {
            '\'' if !double_quoted => {
                single_quoted = !single_quoted;
            }
            '"' if !single_quoted => {
                double_quoted = !double_quoted;
            }
            '\\' if double_quoted => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ch if ch.is_whitespace() && !single_quoted && !double_quoted => {
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}

fn collect_components(
    tree: &ProjectTree,
    root_rel_dir: &str,
    input_failures: &mut Vec<InputFailureFacts>,
) -> Vec<TestComponentFacts> {
    let crates_rel_dir = join_under_root(root_rel_dir, "crates");
    let direct_runtime_rel_dir = ProjectTree::join_rel(&crates_rel_dir, "runtime");
    let direct_runtime_cargo_rel_path =
        ProjectTree::join_rel(&direct_runtime_rel_dir, "Cargo.toml");
    if !tree.file_exists(&direct_runtime_cargo_rel_path) {
        return Vec::new();
    }

    vec![build_component_facts(
        tree,
        root_rel_dir,
        root_rel_dir,
        &direct_runtime_rel_dir,
        input_failures,
    )]
}

fn build_component_facts(
    tree: &ProjectTree,
    root_rel_dir: &str,
    component_rel_dir: &str,
    runtime_rel_dir: &str,
    input_failures: &mut Vec<InputFailureFacts>,
) -> TestComponentFacts {
    let runtime_cargo_rel_path = ProjectTree::join_rel(runtime_rel_dir, "Cargo.toml");
    let runtime_parsed =
        parse_manifest(tree, root_rel_dir, &runtime_cargo_rel_path, input_failures);
    let component_parent = parent_dir(runtime_rel_dir).to_owned();
    let assertions_rel_dir = ProjectTree::join_rel(&component_parent, "assertions");
    let assertions_cargo_rel_path = ProjectTree::join_rel(&assertions_rel_dir, "Cargo.toml");
    let assertions_exists = tree.file_exists(&assertions_cargo_rel_path);
    let assertions_parsed = if assertions_exists {
        parse_manifest(
            tree,
            root_rel_dir,
            &assertions_cargo_rel_path,
            input_failures,
        )
    } else {
        None
    };

    TestComponentFacts {
        rel_dir: component_rel_dir.to_owned(),
        runtime_rel_dir: runtime_rel_dir.to_owned(),
        runtime_cargo_rel_path,
        runtime_package_name: runtime_parsed.as_ref().and_then(manifest_package_name),
        runtime_normal_dependencies: runtime_parsed
            .as_ref()
            .map(manifest_normal_dependencies)
            .unwrap_or_default(),
        runtime_dev_dependencies: runtime_parsed
            .as_ref()
            .map(manifest_dev_dependencies)
            .unwrap_or_default(),
        assertions_rel_dir: assertions_rel_dir.clone(),
        assertions_cargo_rel_path,
        assertions_exists,
        assertions_package_name: assertions_parsed.as_ref().and_then(manifest_package_name),
        assertions_dependencies: assertions_parsed
            .as_ref()
            .map(manifest_normal_dependencies)
            .unwrap_or_default(),
        sidecars: collect_sidecars(tree, runtime_rel_dir, &assertions_rel_dir),
        external_harnesses: collect_external_harnesses(tree, runtime_rel_dir),
    }
}

fn parse_manifest(
    tree: &ProjectTree,
    root_rel_dir: &str,
    rel_path: &str,
    input_failures: &mut Vec<InputFailureFacts>,
) -> Option<toml::Value> {
    let content = match read_cached_or_fs(tree, rel_path) {
        Ok(Some(content)) => content,
        Ok(None) => return None,
        Err(read_error) => {
            input_failures.push(InputFailureFacts {
                root_rel_dir: root_rel_dir.to_owned(),
                rel_path: rel_path.to_owned(),
                message: format!(
                    "Failed to read Cargo.toml for test-family boundaries: {read_error}"
                ),
            });
            return None;
        }
    };
    match toml::from_str::<toml::Value>(&content) {
        Ok(parsed) => Some(parsed),
        Err(parse_error) => {
            input_failures.push(InputFailureFacts {
                root_rel_dir: root_rel_dir.to_owned(),
                rel_path: rel_path.to_owned(),
                message: format!(
                    "Failed to parse Cargo.toml for test-family boundaries: {parse_error}"
                ),
            });
            None
        }
    }
}

fn manifest_package_name(parsed: &toml::Value) -> Option<String> {
    parsed
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
        .map(rust_crate_name)
}

fn manifest_normal_dependencies(parsed: &toml::Value) -> BTreeSet<String> {
    dependency_names(parsed, ["dependencies", "build-dependencies"])
}

fn manifest_dev_dependencies(parsed: &toml::Value) -> BTreeSet<String> {
    dependency_names(parsed, ["dev-dependencies"])
}

fn dependency_names<const N: usize>(parsed: &toml::Value, sections: [&str; N]) -> BTreeSet<String> {
    sections
        .into_iter()
        .filter_map(|section| parsed.get(section).and_then(toml::Value::as_table))
        .flat_map(|table| table.keys().map(|name| rust_crate_name(name)))
        .collect()
}

fn rust_crate_name(package_name: &str) -> String {
    package_name.replace('-', "_")
}

fn collect_sidecars(
    tree: &ProjectTree,
    runtime_rel_dir: &str,
    assertions_rel_dir: &str,
) -> Vec<SidecarFacts> {
    let src_rel_dir = ProjectTree::join_rel(runtime_rel_dir, "src");
    let mut sidecars = Vec::new();

    for dir_rel in tree.all_dir_rels() {
        if !path_is_under(&dir_rel, &src_rel_dir) {
            continue;
        }
        let Some(dir_name) = dir_rel.rsplit('/').next() else {
            continue;
        };
        let Some(owner_module_name) = dir_name.strip_suffix("_tests") else {
            continue;
        };
        let mod_rel_path = ProjectTree::join_rel(&dir_rel, "mod.rs");
        if !tree.file_exists(&mod_rel_path) {
            continue;
        }
        let sidecar_root_rel = dir_rel
            .strip_prefix(&src_rel_dir)
            .and_then(|rest| rest.strip_prefix('/'))
            .unwrap_or(dir_name);
        let relative_parent = parent_dir(sidecar_root_rel);
        let assertions_src_rel = ProjectTree::join_rel(assertions_rel_dir, "src");
        let assertions_module_rel_path = if relative_parent.is_empty() {
            ProjectTree::join_rel(&assertions_src_rel, &format!("{owner_module_name}.rs"))
        } else {
            ProjectTree::join_rel(
                &assertions_src_rel,
                &format!("{relative_parent}/{owner_module_name}.rs"),
            )
        };
        sidecars.push(SidecarFacts {
            mod_rel_path,
            assertions_module_rel_path,
        });
    }

    sidecars.sort_by(|left, right| left.mod_rel_path.cmp(&right.mod_rel_path));
    sidecars
}

fn collect_external_harnesses(tree: &ProjectTree, runtime_rel_dir: &str) -> Vec<String> {
    let tests_rel_dir = ProjectTree::join_rel(runtime_rel_dir, "tests");
    let Some(tests_dir) = tree.dir_contents(&tests_rel_dir) else {
        return Vec::new();
    };

    let mut files = tests_dir
        .files
        .iter()
        .filter(|file_name| file_name.ends_with(".rs"))
        .map(|file_name| ProjectTree::join_rel(&tests_rel_dir, file_name))
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn classify_file(root: &TestRootFacts, rel_path: &str) -> DiscoveredTestFile {
    for component in &root.components {
        let runtime_src = ProjectTree::join_rel(&component.runtime_rel_dir, "src");
        if path_is_under(rel_path, &runtime_src) {
            let rel_after_src = rel_path
                .strip_prefix(&runtime_src)
                .and_then(|rest| rest.strip_prefix('/'))
                .unwrap_or("");
            if rel_after_src.ends_with("_tests/mod.rs") {
                return DiscoveredTestFile {
                    rel_path: rel_path.to_owned(),
                    root_rel_dir: root.rel_dir.clone(),
                    kind: TestFileKind::InternalSidecarMod,
                    owner_module_name: rel_after_src
                        .rsplit_once('/')
                        .and_then(|(parent, _)| parent.rsplit('/').next())
                        .and_then(|segment| segment.strip_suffix("_tests"))
                        .map(str::to_owned),
                    component_rel_dir: Some(component.rel_dir.clone()),
                    assertions_package_name: component.assertions_package_name.clone(),
                };
            }
            if let Some(owner_module_name) = owner_module_name_from_sidecar_path(rel_after_src) {
                return DiscoveredTestFile {
                    rel_path: rel_path.to_owned(),
                    root_rel_dir: root.rel_dir.clone(),
                    kind: TestFileKind::InternalSidecarSupport,
                    owner_module_name: Some(owner_module_name),
                    component_rel_dir: Some(component.rel_dir.clone()),
                    assertions_package_name: component.assertions_package_name.clone(),
                };
            }
            return DiscoveredTestFile {
                rel_path: rel_path.to_owned(),
                root_rel_dir: root.rel_dir.clone(),
                kind: TestFileKind::Source,
                owner_module_name: file_stem(rel_path).map(str::to_owned),
                component_rel_dir: Some(component.rel_dir.clone()),
                assertions_package_name: component.assertions_package_name.clone(),
            };
        }

        for external_harness in &component.external_harnesses {
            if rel_path == external_harness {
                return DiscoveredTestFile {
                    rel_path: rel_path.to_owned(),
                    root_rel_dir: root.rel_dir.clone(),
                    kind: TestFileKind::ExternalHarness,
                    owner_module_name: None,
                    component_rel_dir: Some(component.rel_dir.clone()),
                    assertions_package_name: component.assertions_package_name.clone(),
                };
            }
        }

        let assertions_src = ProjectTree::join_rel(&component.assertions_rel_dir, "src");
        if path_is_under(rel_path, &assertions_src) {
            return DiscoveredTestFile {
                rel_path: rel_path.to_owned(),
                root_rel_dir: root.rel_dir.clone(),
                kind: TestFileKind::AssertionsModule,
                owner_module_name: file_stem(rel_path).map(str::to_owned),
                component_rel_dir: Some(component.rel_dir.clone()),
                assertions_package_name: component.assertions_package_name.clone(),
            };
        }
    }

    let root_relative = root_relative(rel_path, &root.rel_dir);
    if let Some(rel_after_src) = rel_after_named_dir(root_relative, "src") {
        if rel_after_src.ends_with("_tests/mod.rs") {
            return DiscoveredTestFile {
                rel_path: rel_path.to_owned(),
                root_rel_dir: root.rel_dir.clone(),
                kind: TestFileKind::InternalSidecarMod,
                owner_module_name: rel_after_src
                    .rsplit_once('/')
                    .and_then(|(parent, _)| parent.rsplit('/').next())
                    .and_then(|segment| segment.strip_suffix("_tests"))
                    .map(str::to_owned),
                component_rel_dir: None,
                assertions_package_name: None,
            };
        }
        if let Some(owner_module_name) = owner_module_name_from_sidecar_path(rel_after_src) {
            return DiscoveredTestFile {
                rel_path: rel_path.to_owned(),
                root_rel_dir: root.rel_dir.clone(),
                kind: TestFileKind::InternalSidecarSupport,
                owner_module_name: Some(owner_module_name),
                component_rel_dir: None,
                assertions_package_name: None,
            };
        }
        return DiscoveredTestFile {
            rel_path: rel_path.to_owned(),
            root_rel_dir: root.rel_dir.clone(),
            kind: TestFileKind::Source,
            owner_module_name: file_stem(rel_path).map(str::to_owned),
            component_rel_dir: None,
            assertions_package_name: None,
        };
    }
    let kind = if rel_after_named_dir(root_relative, "tests").is_some() {
        TestFileKind::ExternalHarness
    } else {
        TestFileKind::Other
    };
    DiscoveredTestFile {
        rel_path: rel_path.to_owned(),
        root_rel_dir: root.rel_dir.clone(),
        kind,
        owner_module_name: file_stem(rel_path).map(str::to_owned),
        component_rel_dir: None,
        assertions_package_name: None,
    }
}

fn owner_module_name_from_sidecar_path(rel_after_src: &str) -> Option<String> {
    rel_after_src.split('/').find_map(|segment| {
        segment
            .strip_suffix("_tests")
            .map(str::to_owned)
            .filter(|value| !value.is_empty())
    })
}

fn rel_after_named_dir<'a>(root_relative: &'a str, dir_name: &str) -> Option<&'a str> {
    let prefix = format!("{dir_name}/");
    if let Some(rest) = root_relative.strip_prefix(&prefix) {
        return Some(rest);
    }
    let marker = format!("/{dir_name}/");
    root_relative.rsplit_once(&marker).map(|(_, rest)| rest)
}

fn read_cached_or_fs(tree: &ProjectTree, rel_path: &str) -> Result<Option<String>, std::io::Error> {
    if let Some(content) = tree.file_content(rel_path) {
        return Ok(Some(content.to_owned()));
    }
    if !tree.file_exists(rel_path) {
        return Ok(None);
    }
    guardrail3_shared_fs::read_file_err(&tree.abs_path(rel_path)).map(Some)
}
