use std::collections::{BTreeMap, BTreeSet};

use guardrail3_domain_report::{CheckResult, Severity};

use super::AnalyzedFile;
use super::discover::{parent_dir, path_is_under};
use super::facts::{RuntimeAssertionsViolation, TestFileKind, TestRootFacts};
use super::inputs::RuntimeAssertionsViolationInput;
use super::parse::{ModuleInfo, UseBinding};

const ID: &str = "RS-TEST-03";
pub(crate) fn collect(
    root: &TestRootFacts,
    files: &[AnalyzedFile],
    scoped_files: Option<&BTreeSet<String>>,
    local_package_names: &BTreeSet<String>,
    results: &mut Vec<CheckResult>,
) {
    for violation in collect_violations(root, files, scoped_files, local_package_names) {
        check(&RuntimeAssertionsViolationInput::new(&violation), results);
    }
}

pub fn check(input: &RuntimeAssertionsViolationInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: input.violation.title.clone(),
        message: input.violation.message.clone(),
        file: Some(input.violation.rel_path.clone()),
        line: input.violation.line,
        inventory: false,
    });
}

fn collect_violations(
    root: &TestRootFacts,
    files: &[AnalyzedFile],
    scoped_files: Option<&BTreeSet<String>>,
    local_package_names: &BTreeSet<String>,
) -> Vec<RuntimeAssertionsViolation> {
    let mut violations = Vec::new();
    let parsed_by_path = files
        .iter()
        .map(|file| (file.facts.rel_path.clone(), file))
        .collect::<BTreeMap<_, _>>();

    violations.extend(non_component_harness_violations(files, scoped_files));

    for component in &root.components {
        let harnesses_exist =
            !component.sidecars.is_empty() || !component.external_harnesses.is_empty();
        if !harnesses_exist {
            continue;
        }
        let allowed_external_packages = allowed_external_harness_packages(component);
        let allowed_sidecar_packages = allowed_sidecar_packages(component);
        let allowed_assertions_packages = allowed_assertions_packages(component);
        let assertions_package_name = component.assertions_package_name.as_deref();

        if !component.assertions_exists {
            violations.push(RuntimeAssertionsViolation {
                rel_path: component.assertions_cargo_rel_path.clone(),
                line: None,
                title: "assertions crate missing".to_owned(),
                message: format!(
                    "Component `{}` has test harnesses but is missing sibling `assertions/Cargo.toml`.",
                    component.rel_dir
                ),
            });
        }

        if let Some(assertions_package_name) = component.assertions_package_name.as_ref() {
            if component
                .runtime_normal_dependencies
                .contains(assertions_package_name)
            {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: component.runtime_cargo_rel_path.clone(),
                    line: None,
                    title: "runtime depends on assertions at normal scope".to_owned(),
                    message: "Runtime crates must not take sibling assertions crates as normal dependencies.".to_owned(),
                });
            }
            if !component
                .runtime_dev_dependencies
                .contains(assertions_package_name)
            {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: component.runtime_cargo_rel_path.clone(),
                    line: None,
                    title: "runtime missing assertions dev-dependency".to_owned(),
                    message: "Runtime crates with test harnesses must wire sibling assertions crates through `dev-dependencies`.".to_owned(),
                });
            }
        }

        if component.assertions_exists
            && component
                .runtime_package_name
                .as_ref()
                .is_some_and(|runtime_package_name| {
                    !component
                        .assertions_dependencies
                        .contains(runtime_package_name)
                })
        {
            violations.push(RuntimeAssertionsViolation {
                rel_path: component.assertions_cargo_rel_path.clone(),
                line: None,
                title: "assertions missing runtime dependency".to_owned(),
                message:
                    "Assertions crates must depend on the sibling runtime crate they validate."
                        .to_owned(),
            });
        }

        for sidecar in &component.sidecars {
            if scoped_files.is_some_and(|paths| !paths.contains(&sidecar.mod_rel_path)) {
                continue;
            }
            if !root_has_file(files, &sidecar.assertions_module_rel_path) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: sidecar.mod_rel_path.clone(),
                    line: None,
                    title: "sidecar missing owned assertions module".to_owned(),
                    message: format!(
                        "Owned sidecar `{}` requires reusable assertions module `{}`.",
                        sidecar.mod_rel_path, sidecar.assertions_module_rel_path
                    ),
                });
            }
        }

        for external_harness in &component.external_harnesses {
            if scoped_files.is_some_and(|paths| !paths.contains(external_harness)) {
                continue;
            }
            let Some(file) = parsed_by_path.get(external_harness) else {
                continue;
            };

            for binding in &file.parsed.imports {
                if import_uses_external_runtime_boundary(binding) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: external_harness.clone(),
                        line: Some(binding.line),
                        title: "external harness reaches private runtime glue".to_owned(),
                        message: "External runtime harnesses must stay black-box and must not import `crate::` or `super::`.".to_owned(),
                    });
                }
                if let Some(local_root) = first_disallowed_local_package(
                    &binding.path_segments,
                    local_package_names,
                    &allowed_external_packages,
                ) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: external_harness.clone(),
                        line: Some(binding.line),
                        title: "external harness imports disallowed local crate".to_owned(),
                        message: format!(
                            "External runtime harnesses must stay black-box and must not import local crate `{local_root}` directly."
                        ),
                    });
                }
            }

            for module in &file.parsed.modules {
                if module_path_includes_runtime_src(
                    module,
                    external_harness,
                    &component.runtime_rel_dir,
                ) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: external_harness.clone(),
                        line: Some(module.line),
                        title: "external harness path-includes runtime source".to_owned(),
                        message: "External runtime harnesses must not path-include files from runtime `src/`.".to_owned(),
                    });
                }
            }

            if let Some(local_root) = file.parsed.file_call_paths.iter().find_map(|path| {
                first_disallowed_local_package(
                    path,
                    local_package_names,
                    &allowed_external_packages,
                )
                .map(str::to_owned)
            }) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: external_harness.clone(),
                    line: None,
                    title: "external harness calls disallowed local crate".to_owned(),
                    message: format!(
                        "External runtime harnesses must stay black-box and must not call local crate `{}` directly.",
                        local_root
                    ),
                });
            }
        }

        for file in files.iter().filter(|file| {
            file.facts.component_rel_dir.as_deref() == Some(component.rel_dir.as_str())
                && matches!(
                    file.facts.kind,
                    TestFileKind::InternalSidecarMod | TestFileKind::InternalSidecarSupport
                )
        }) {
            if scoped_files.is_some_and(|paths| !paths.contains(&file.facts.rel_path)) {
                continue;
            }
            let Some(owner_module_name) = file.facts.owner_module_name.as_deref() else {
                continue;
            };
            let local_module_names = files
                .iter()
                .filter(|candidate| {
                    candidate.facts.component_rel_dir.as_deref() == Some(component.rel_dir.as_str())
                        && matches!(candidate.facts.kind, TestFileKind::Source)
                })
                .filter_map(|candidate| candidate.facts.owner_module_name.clone())
                .collect::<BTreeSet<_>>();
            for binding in &file.parsed.imports {
                if let Some(target) = disallowed_sidecar_local_boundary_target(
                    &binding.path_segments,
                    &file.facts.kind,
                    owner_module_name,
                    &local_module_names,
                ) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.facts.rel_path.clone(),
                        line: Some(binding.line),
                        title: "sidecar escapes owned module boundary".to_owned(),
                        message: format!(
                            "Internal sidecar harnesses must not escape their owned module boundary through local path `{target}`."
                        ),
                    });
                }
                if import_hits_sibling_module(binding, owner_module_name, &local_module_names) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.facts.rel_path.clone(),
                        line: Some(binding.line),
                        title: "sidecar imports sibling production module".to_owned(),
                        message: "Internal sidecar harnesses may reach only their owned production module subtree, not sibling production modules.".to_owned(),
                    });
                }
                if let Some(local_root) = first_disallowed_local_package(
                    &binding.path_segments,
                    local_package_names,
                    &allowed_sidecar_packages,
                ) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.facts.rel_path.clone(),
                        line: Some(binding.line),
                        title: "sidecar imports disallowed local crate".to_owned(),
                        message: format!(
                            "Internal sidecar harnesses must not import local crate `{local_root}` directly."
                        ),
                    });
                }
                if let Some(target_module) = foreign_assertions_module_target(
                    &binding.path_segments,
                    assertions_package_name,
                    owner_module_name,
                ) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.facts.rel_path.clone(),
                        line: Some(binding.line),
                        title: "sidecar imports sibling assertions module".to_owned(),
                        message: format!(
                            "Internal sidecar harnesses may only import owned assertions module `{owner_module_name}` and must not import sibling assertions module `{target_module}`."
                        ),
                    });
                }
            }
            if let Some(target) = file.parsed.file_call_paths.iter().find_map(|path| {
                disallowed_sidecar_local_boundary_target(
                    path,
                    &file.facts.kind,
                    owner_module_name,
                    &local_module_names,
                )
            }) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.facts.rel_path.clone(),
                    line: None,
                    title: "sidecar escapes owned module boundary".to_owned(),
                    message: format!(
                        "Internal sidecar harnesses must not escape their owned module boundary through local call path `{target}`."
                    ),
                });
            }
            if let Some(target_module) = file.parsed.file_call_paths.iter().find_map(|path| {
                sibling_module_target(path, owner_module_name, &local_module_names)
            }) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.facts.rel_path.clone(),
                    line: None,
                    title: "sidecar calls sibling production module".to_owned(),
                    message: format!(
                        "Internal sidecar harnesses must not call sibling production module `{target_module}` directly."
                    ),
                });
            }
            if let Some(local_root) = file.parsed.file_call_paths.iter().find_map(|path| {
                first_disallowed_local_package(path, local_package_names, &allowed_sidecar_packages)
                    .map(str::to_owned)
            }) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.facts.rel_path.clone(),
                    line: None,
                    title: "sidecar calls disallowed local crate".to_owned(),
                    message: format!(
                        "Internal sidecar harnesses must not call local crate `{}` directly.",
                        local_root
                    ),
                });
            }
            if let Some(target_module) = file.parsed.file_call_paths.iter().find_map(|path| {
                foreign_assertions_module_target(path, assertions_package_name, owner_module_name)
            }) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.facts.rel_path.clone(),
                    line: None,
                    title: "sidecar calls sibling assertions module".to_owned(),
                    message: format!(
                        "Internal sidecar harnesses may only call owned assertions module `{owner_module_name}` and must not call sibling assertions module `{target_module}`."
                    ),
                });
            }
        }

        for file in files.iter().filter(|file| {
            file.facts.component_rel_dir.as_deref() == Some(component.rel_dir.as_str())
                && matches!(file.facts.kind, TestFileKind::AssertionsModule)
        }) {
            if scoped_files.is_some_and(|paths| !paths.contains(&file.facts.rel_path)) {
                continue;
            }
            for binding in &file.parsed.imports {
                if import_uses_local_boundary(binding) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.facts.rel_path.clone(),
                        line: Some(binding.line),
                        title: "assertions module reaches local private code".to_owned(),
                        message: "Assertions modules must import runtime public API or helper crates, not local `crate::`, `self::`, or `super::` paths.".to_owned(),
                    });
                }
                if let Some(local_root) = first_disallowed_local_package(
                    &binding.path_segments,
                    local_package_names,
                    &allowed_assertions_packages,
                ) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.facts.rel_path.clone(),
                        line: Some(binding.line),
                        title: "assertions module imports disallowed local crate".to_owned(),
                        message: format!(
                            "Assertions modules must not import local crate `{local_root}` directly."
                        ),
                    });
                }
                if binding
                    .path_segments
                    .iter()
                    .any(|segment| segment == "FamilyMapper")
                {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.facts.rel_path.clone(),
                        line: Some(binding.line),
                        title: "assertions module imports route construction infrastructure"
                            .to_owned(),
                        message:
                            "Assertions modules must stay reusable semantic proof helpers and must not import route-construction infrastructure."
                                .to_owned(),
                    });
                }
            }
            if let Some(local_root) = file.parsed.file_call_paths.iter().find_map(|path| {
                first_disallowed_local_package(
                    path,
                    local_package_names,
                    &allowed_assertions_packages,
                )
                .map(str::to_owned)
            }) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.facts.rel_path.clone(),
                    line: None,
                    title: "assertions module calls disallowed local crate".to_owned(),
                    message: format!(
                        "Assertions modules must not call local crate `{}` directly.",
                        local_root
                    ),
                });
            }
            if file
                .parsed
                .file_call_paths
                .iter()
                .any(|call_path| call_path.iter().any(|segment| segment == "FamilyMapper"))
            {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.facts.rel_path.clone(),
                    line: None,
                    title: "assertions module builds routed family input".to_owned(),
                    message: "Assertions modules must stay reusable semantic proof helpers and must not construct routed family inputs through `FamilyMapper`.".to_owned(),
                });
            }
            if assertions_call_runtime_check_test_tree(
                &file.parsed.imports,
                &file.parsed.file_call_paths,
                component.runtime_package_name.as_deref(),
            ) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.facts.rel_path.clone(),
                    line: None,
                    title: "assertions module orchestrates family execution".to_owned(),
                    message: "Assertions modules must not call runtime `check_test_tree(...)`; sidecars own family execution and assertions own reusable semantic proof only.".to_owned(),
                });
            }
        }
    }

    violations.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    violations
}

fn non_component_harness_violations(
    files: &[AnalyzedFile],
    scoped_files: Option<&BTreeSet<String>>,
) -> Vec<RuntimeAssertionsViolation> {
    files.iter()
        .filter(|file| file.facts.component_rel_dir.is_none())
        .filter(|file| {
            matches!(
                file.facts.kind,
                TestFileKind::InternalSidecarMod | TestFileKind::ExternalHarness
            )
        })
        .filter(|file| scoped_files.is_none_or(|paths| paths.contains(&file.facts.rel_path)))
        .map(|file| RuntimeAssertionsViolation {
            rel_path: file.facts.rel_path.clone(),
            line: None,
            title: "test harness outside runtime/assertions split".to_owned(),
            message: "Test harnesses must live under a discovered `runtime` crate with a sibling `assertions` crate; plain root-local sidecars and external harnesses are not allowed.".to_owned(),
        })
        .collect()
}

fn root_has_file(files: &[AnalyzedFile], rel_path: &str) -> bool {
    files.iter().any(|file| file.facts.rel_path == rel_path)
}

fn import_uses_external_runtime_boundary(binding: &UseBinding) -> bool {
    binding
        .path_segments
        .first()
        .is_some_and(|segment| matches!(segment.as_str(), "crate" | "super"))
}

fn import_uses_local_boundary(binding: &UseBinding) -> bool {
    binding
        .path_segments
        .first()
        .is_some_and(|segment| matches!(segment.as_str(), "crate" | "self" | "super"))
}

fn assertions_call_runtime_check_test_tree(
    imports: &[UseBinding],
    call_paths: &[Vec<String>],
    runtime_package_name: Option<&str>,
) -> bool {
    let Some(runtime_package_name) = runtime_package_name else {
        return false;
    };
    let mut runtime_roots = BTreeSet::from([
        runtime_package_name.to_owned(),
        runtime_package_name.replace('-', "_"),
    ]);
    let mut imported_check_test_tree = BTreeSet::new();

    for binding in imports {
        if binding
            .path_segments
            .first()
            .is_none_or(|first| !runtime_roots.contains(first))
        {
            continue;
        }
        if let Some(local_name) = binding.local_name.as_ref() {
            if binding.path_segments.len() == 1 {
                let _ = runtime_roots.insert(local_name.clone());
            } else if binding
                .path_segments
                .last()
                .is_some_and(|segment| segment == "check_test_tree")
            {
                let _ = imported_check_test_tree.insert(local_name.clone());
            }
        }
    }

    call_paths.iter().any(|path| match path.as_slice() {
        [single] => imported_check_test_tree.contains(single),
        [first, second, ..] => runtime_roots.contains(first) && second == "check_test_tree",
        _ => false,
    })
}

fn import_hits_sibling_module(
    binding: &UseBinding,
    owner_module_name: &str,
    local_module_names: &BTreeSet<String>,
) -> bool {
    sibling_module_target(
        &binding.path_segments,
        owner_module_name,
        local_module_names,
    )
    .is_some()
}

fn sibling_module_target<'a>(
    path_segments: &'a [String],
    owner_module_name: &str,
    local_module_names: &BTreeSet<String>,
) -> Option<&'a str> {
    let imported = first_local_module_target(path_segments)?;
    if !local_module_names.contains(imported) {
        return None;
    }
    let owner_tests_module_name = format!("{owner_module_name}_tests");
    if imported == owner_module_name || imported == owner_tests_module_name {
        return None;
    }
    Some(imported)
}

fn disallowed_sidecar_local_boundary_target(
    path_segments: &[String],
    file_kind: &TestFileKind,
    owner_module_name: &str,
    local_module_names: &BTreeSet<String>,
) -> Option<String> {
    let Some(first) = path_segments.first() else {
        return None;
    };
    let owner_tests_module_name = format!("{owner_module_name}_tests");
    match first.as_str() {
        "crate" => {
            let target = path_segments.get(1)?;
            if target == owner_module_name
                || target == &owner_tests_module_name
                || local_module_names.contains(target)
            {
                None
            } else {
                Some(target.clone())
            }
        }
        "self" | "super" => {
            let boundary_depth = path_segments
                .iter()
                .take_while(|segment| matches!(segment.as_str(), "self" | "super"))
                .count();
            let max_depth = match file_kind {
                TestFileKind::InternalSidecarMod => 1,
                TestFileKind::InternalSidecarSupport => 2,
                _ => 0,
            };
            (boundary_depth > max_depth).then(|| {
                path_segments
                    .get(boundary_depth)
                    .cloned()
                    .unwrap_or_else(|| "<crate-root>".to_owned())
            })
        }
        _ => None,
    }
}

fn first_local_module_target(path_segments: &[String]) -> Option<&str> {
    let first = path_segments.first()?;
    match first.as_str() {
        "crate" => path_segments.get(1).map(String::as_str),
        "self" | "super" => path_segments
            .iter()
            .skip(1)
            .find(|segment| !matches!(segment.as_str(), "self" | "super"))
            .map(String::as_str),
        _ => None,
    }
}

fn allowed_external_harness_packages(
    component: &super::facts::TestComponentFacts,
) -> BTreeSet<String> {
    let mut allowed = BTreeSet::from(["test_support".to_owned()]);
    if let Some(runtime_package_name) = component.runtime_package_name.as_ref() {
        let _ = allowed.insert(runtime_package_name.clone());
    }
    if let Some(assertions_package_name) = component.assertions_package_name.as_ref() {
        let _ = allowed.insert(assertions_package_name.clone());
    }
    allowed
}

fn allowed_sidecar_packages(component: &super::facts::TestComponentFacts) -> BTreeSet<String> {
    let mut allowed = BTreeSet::from(["test_support".to_owned()]);
    if let Some(assertions_package_name) = component.assertions_package_name.as_ref() {
        let _ = allowed.insert(assertions_package_name.clone());
    }
    allowed
}

fn allowed_assertions_packages(component: &super::facts::TestComponentFacts) -> BTreeSet<String> {
    let mut allowed = BTreeSet::from(["test_support".to_owned()]);
    if let Some(runtime_package_name) = component.runtime_package_name.as_ref() {
        let _ = allowed.insert(runtime_package_name.clone());
    }
    if let Some(assertions_package_name) = component.assertions_package_name.as_ref() {
        let _ = allowed.insert(format!("{assertions_package_name}_common"));
    }
    allowed
}

fn first_disallowed_local_package<'a>(
    path: &'a [String],
    local_package_names: &'a BTreeSet<String>,
    allowed_local_packages: &'a BTreeSet<String>,
) -> Option<&'a str> {
    let root = path.first()?;
    if !local_package_names.contains(root) || allowed_local_packages.contains(root) {
        return None;
    }
    Some(root.as_str())
}

fn foreign_assertions_module_target<'a>(
    path_segments: &'a [String],
    assertions_package_name: Option<&str>,
    owner_module_name: &str,
) -> Option<&'a str> {
    let assertions_package_name = assertions_package_name?;
    let [first, second, ..] = path_segments else {
        return None;
    };
    if first != assertions_package_name || second == owner_module_name {
        return None;
    }
    Some(second.as_str())
}

fn module_path_includes_runtime_src(
    module: &ModuleInfo,
    file_rel_path: &str,
    runtime_rel_dir: &str,
) -> bool {
    let Some(path_attr) = module.path_attr.as_deref() else {
        return false;
    };
    let file_dir = parent_dir(file_rel_path);
    let Some(resolved) = resolve_relative_path(file_dir, path_attr) else {
        return false;
    };
    path_is_under(&resolved, &format!("{runtime_rel_dir}/src"))
}

fn resolve_relative_path(base_dir: &str, rel_path: &str) -> Option<String> {
    let mut parts = if base_dir.is_empty() {
        Vec::new()
    } else {
        base_dir.split('/').map(str::to_owned).collect::<Vec<_>>()
    };
    for part in rel_path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                let _ = parts.pop()?;
            }
            value => parts.push(value.to_owned()),
        }
    }
    Some(parts.join("/"))
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    super::check_test_tree(&tree, &test_support::StubToolChecker::default())
}

#[cfg(test)]
#[allow(dead_code)]
#[allow(dead_code)]
pub(crate) fn run_family_with_tool(
    root: &std::path::Path,
    cargo_mutants_installed: bool,
) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    let checker = if cargo_mutants_installed {
        test_support::StubToolChecker::with_tools(["cargo-mutants"])
    } else {
        test_support::StubToolChecker::default()
    };
    super::check_test_tree(&tree, &checker)
}

#[cfg(test)]
#[path = "rs_test_03_runtime_assertions_split_tests/mod.rs"]
mod rs_test_03_runtime_assertions_split_tests;
