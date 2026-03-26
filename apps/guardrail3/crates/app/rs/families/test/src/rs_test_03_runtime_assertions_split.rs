use std::collections::{BTreeMap, BTreeSet};

use guardrail3_domain_report::{CheckResult, Severity};

use super::discover::{parent_dir, path_is_under};
use super::facts::{RuntimeAssertionsViolation, TestFileKind, TestRootFacts};
use super::inputs::RuntimeAssertionsViolationInput;
use super::parse::{ModuleInfo, UseBinding};
use super::AnalyzedFile;

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
        let harnesses_exist = !component.sidecars.is_empty() || !component.external_harnesses.is_empty();
        if !harnesses_exist {
            continue;
        }
        let allowed_external_packages = allowed_external_harness_packages(component);
        let allowed_sidecar_packages = allowed_sidecar_packages(component);
        let allowed_assertions_packages = allowed_assertions_packages(component);

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
            if component.runtime_normal_dependencies.contains(assertions_package_name) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: component.runtime_cargo_rel_path.clone(),
                    line: None,
                    title: "runtime depends on assertions at normal scope".to_owned(),
                    message: "Runtime crates must not take sibling assertions crates as normal dependencies.".to_owned(),
                });
            }
            if !component.runtime_dev_dependencies.contains(assertions_package_name) {
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
                    !component.assertions_dependencies.contains(runtime_package_name)
                })
        {
            violations.push(RuntimeAssertionsViolation {
                rel_path: component.assertions_cargo_rel_path.clone(),
                line: None,
                title: "assertions missing runtime dependency".to_owned(),
                message: "Assertions crates must depend on the sibling runtime crate they validate.".to_owned(),
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
                if module_path_includes_runtime_src(module, external_harness, &component.runtime_rel_dir) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: external_harness.clone(),
                        line: Some(module.line),
                        title: "external harness path-includes runtime source".to_owned(),
                        message: "External runtime harnesses must not path-include files from runtime `src/`.".to_owned(),
                    });
                }
            }

            if let Some(local_root) = file
                .parsed
                .file_call_paths
                .iter()
                .find_map(|path| {
                    first_disallowed_local_package(
                        path,
                        local_package_names,
                        &allowed_external_packages,
                    )
                    .map(str::to_owned)
                })
            {
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
            for binding in &file.parsed.imports {
                if import_hits_sibling_module(binding, owner_module_name) {
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
            }
            if let Some(local_root) = file
                .parsed
                .file_call_paths
                .iter()
                .find_map(|path| {
                    first_disallowed_local_package(
                        path,
                        local_package_names,
                        &allowed_sidecar_packages,
                    )
                    .map(str::to_owned)
                })
            {
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
            }
            if let Some(local_root) = file
                .parsed
                .file_call_paths
                .iter()
                .find_map(|path| {
                    first_disallowed_local_package(
                        path,
                        local_package_names,
                        &allowed_assertions_packages,
                    )
                    .map(str::to_owned)
                })
            {
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

fn import_hits_sibling_module(binding: &UseBinding, owner_module_name: &str) -> bool {
    let Some(first) = binding.path_segments.first() else {
        return false;
    };
    if first != "crate" {
        return false;
    }
    let Some(imported) = binding.path_segments.get(1) else {
        return true;
    };
    imported != owner_module_name && imported != &format!("{owner_module_name}_tests")
}

fn allowed_external_harness_packages(component: &super::facts::TestComponentFacts) -> BTreeSet<String> {
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
#[path = "rs_test_03_runtime_assertions_split_tests/mod.rs"]
mod tests;
