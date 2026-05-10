#![expect(
    clippy::shadow_unrelated,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::too_many_lines,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use std::collections::BTreeSet;

use g3rs_test_types::{
    G3RsTestAnalyzedSourceFile, G3RsTestComponentFileTreeFacts, G3RsTestFileKind,
    G3RsTestFileTreeChecksInput,
};

use super::assertions_violations::collect_assertions_module_violations;
use super::helpers;

/// `RuntimeAssertionsViolation` struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RuntimeAssertionsViolation {
    /// `rel_path` item.
    pub(super) rel_path: String,
    /// `line` item.
    pub(super) line: Option<usize>,
    /// `title` item.
    pub(super) title: String,
    /// `message` item.
    pub(super) message: String,
}

/// `collect_violations` function.
pub(super) fn collect_violations(
    input: &G3RsTestFileTreeChecksInput,
) -> Vec<RuntimeAssertionsViolation> {
    let mut violations = Vec::new();

    violations.extend(non_component_harness_violations(&input.files));

    for component in &input.components {
        let harnesses_exist =
            !component.sidecars.is_empty() || !component.external_harnesses.is_empty();
        if !harnesses_exist {
            continue;
        }
        let allowed_external_packages = helpers::allowed_external_harness_packages(component);
        let allowed_sidecar_packages = helpers::allowed_sidecar_packages(component);
        let allowed_assertions_packages = helpers::allowed_assertions_packages(component);
        let assertions_package_name = component.assertions_package_name.as_deref();

        if !component.assertions_exists {
            let component_rel_dir = component_package_rel_dir(component);
            if let Some(nested_assertions_cargo_rel_path) =
                component.nested_assertions_cargo_rel_path.as_ref()
            {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: nested_assertions_cargo_rel_path.clone(),
                    line: None,
                    title: "nested assertions package is the wrong shape".to_owned(),
                    message: format!(
                        "Found nested package `{nested_assertions_cargo_rel_path}`. This is the wrong test layout. If assertions is a separate crate, move it to `{component_rel_dir}/crates/assertions/Cargo.toml` and move the production crate to `{component_rel_dir}/crates/runtime/Cargo.toml` so both are sibling member crates in one package."
                    ),
                });
                continue;
            }
            violations.push(RuntimeAssertionsViolation {
                rel_path: component.assertions_cargo_rel_path.clone(),
                line: None,
                title: "assertions crate missing".to_owned(),
                message: format!(
                    "Component `{component_rel_dir}` has sidecar tests that require a shared assertions crate, but `{component_rel_dir}` is still a single-crate package. Reshape it into one package with sibling member crates under `crates/`: `crates/runtime` for the production crate and `crates/assertions` for shared test proof. Do not add `{component_rel_dir}/assertions/Cargo.toml` directly under the current crate root, because that creates a nested package instead of sibling member crates."
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
                    message: format!(
                        "Manifest `{}` is missing dev-dependency `{}`. Add `{}` under `[dev-dependencies]`, so sidecars and external harnesses can call the shared assertions crate.",
                        component.runtime_cargo_rel_path,
                        assertions_package_name,
                        assertions_package_name,
                    ),
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
                message: format!(
                    "Manifest `{}` is missing dependency `{}`. Add `{}` under `[dependencies]`, so the shared assertions crate can prove the runtime behavior it checks.",
                    component.assertions_cargo_rel_path,
                    component.runtime_package_name.as_deref().unwrap_or("<runtime-package>"),
                    component.runtime_package_name.as_deref().unwrap_or("<runtime-package>"),
                ),
            });
        }

        for sidecar in &component.sidecars {
            if !input
                .existing_file_paths
                .contains(&sidecar.assertions_module_rel_path)
            {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: sidecar.mod_rel_path.clone(),
                    line: None,
                    title: "sidecar missing owned assertions module".to_owned(),
                    message: format!(
                        "Sidecar file `{}` has tests but no shared assertions file `{}`. Create that assertions file and move the final result assertions there, so internal and external tests use the same proof.",
                        sidecar.mod_rel_path, sidecar.assertions_module_rel_path
                    ),
                });
            }
        }

        collect_external_harness_violations(
            &mut violations,
            component,
            &input.local_package_names,
            &allowed_external_packages,
        );
        collect_sidecar_violations(
            &mut violations,
            component,
            &input.local_package_names,
            assertions_package_name,
            &allowed_sidecar_packages,
        );
        collect_assertions_module_violations(
            &mut violations,
            component,
            &input.local_package_names,
            &allowed_assertions_packages,
        );
    }

    violations.sort_by(|left, right| {
        left.rel_path
            .cmp(&right.rel_path)
            .then_with(|| left.line.cmp(&right.line))
            .then_with(|| left.title.cmp(&right.title))
    });
    violations
}

/// `component_package_rel_dir` function.
fn component_package_rel_dir(component: &G3RsTestComponentFileTreeFacts) -> &str {
    if component.rel_dir.is_empty() {
        component.runtime_rel_dir.as_str()
    } else {
        component.rel_dir.as_str()
    }
}

/// `collect_external_harness_violations` function.
fn collect_external_harness_violations(
    violations: &mut Vec<RuntimeAssertionsViolation>,
    component: &G3RsTestComponentFileTreeFacts,
    local_package_names: &BTreeSet<String>,
    allowed_external_packages: &BTreeSet<String>,
) {
    for file in &component.external_harness_files {
        let external_harness = &file.rel_path;
        for binding in &file.parsed.imports {
            if helpers::import_uses_external_runtime_boundary(binding) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: external_harness.clone(),
                    line: Some(binding.line),
                    title: "external harness reaches private runtime glue".to_owned(),
                    message: "External runtime harnesses must stay black-box and must not import `crate::` or `super::`.".to_owned(),
                });
            }
            if let Some(local_root) = helpers::first_disallowed_local_package(
                &binding.path_segments,
                local_package_names,
                allowed_external_packages,
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
            if module.path_attr.is_some() {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: external_harness.clone(),
                    line: Some(module.line),
                    title: "external harness path-includes local source".to_owned(),
                    message: "External runtime harnesses must not use `#[path = ...]` to pull in local source files.".to_owned(),
                });
            }
        }

        if let Some(local_root) = file.parsed.file_call_paths.iter().find_map(|path| {
            helpers::first_disallowed_local_package(
                path,
                local_package_names,
                allowed_external_packages,
            )
            .map(str::to_owned)
        }) {
            violations.push(RuntimeAssertionsViolation {
                rel_path: external_harness.clone(),
                line: None,
                title: "external harness calls disallowed local crate".to_owned(),
                message: format!(
                    "External runtime harnesses must stay black-box and must not call local crate `{local_root}` directly."
                ),
            });
        }
    }
}

/// `collect_sidecar_violations` function.
fn collect_sidecar_violations(
    violations: &mut Vec<RuntimeAssertionsViolation>,
    component: &G3RsTestComponentFileTreeFacts,
    local_package_names: &BTreeSet<String>,
    assertions_package_name: Option<&str>,
    allowed_sidecar_packages: &BTreeSet<String>,
) {
    for file in component.sidecar_files.iter().filter(|file| {
        matches!(
            file.kind,
            G3RsTestFileKind::InternalSidecarMod | G3RsTestFileKind::InternalSidecarSupport
        )
    }) {
        let Some(owner_module_name) = file.owner_module_name.as_deref() else {
            continue;
        };
        let local_module_names = &component.source_module_names;
        for binding in &file.parsed.imports {
            if let Some(target) = helpers::disallowed_sidecar_local_boundary_target(
                &binding.path_segments,
                &file.kind,
                owner_module_name,
                local_module_names,
            ) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.rel_path.clone(),
                    line: Some(binding.line),
                    title: "sidecar escapes owned module boundary".to_owned(),
                    message: format!(
                        "Sidecar file `{}` reaches local path `{}`. Call only the owned production module `{}` or the shared assertions crate from this sidecar, so the sidecar tests one module without reaching into siblings.",
                        file.rel_path,
                        target,
                        owner_module_name,
                    ),
                });
            }
            if helpers::import_hits_sibling_module(binding, owner_module_name, local_module_names) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.rel_path.clone(),
                    line: Some(binding.line),
                    title: "sidecar imports sibling local module".to_owned(),
                    message: format!(
                        "Sidecar file `{}` imports sibling local module `{}`. Import only the owned production module `{}` or the shared assertions crate from this sidecar, so the sidecar tests one module without reaching into siblings.",
                        file.rel_path,
                        helpers::sibling_module_target(
                            &binding.path_segments,
                            owner_module_name,
                            local_module_names,
                        )
                        .unwrap_or("<sibling-module>"),
                        owner_module_name,
                    ),
                });
            }
            if let Some(local_root) = helpers::first_disallowed_local_package(
                &binding.path_segments,
                local_package_names,
                allowed_sidecar_packages,
            ) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.rel_path.clone(),
                    line: Some(binding.line),
                    title: "sidecar imports disallowed local crate".to_owned(),
                    message: format!(
                        "Internal sidecar harnesses must not import local crate `{local_root}` directly."
                    ),
                });
            }
            if let Some(target_module) = helpers::foreign_assertions_module_target(
                &binding.path_segments,
                assertions_package_name,
                &file.rel_path,
                owner_module_name,
            ) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.rel_path.clone(),
                    line: Some(binding.line),
                    title: "sidecar imports sibling assertions module".to_owned(),
                    message: format!(
                        "Internal sidecar harnesses may only import owned assertions module `{owner_module_name}` and must not import sibling assertions module `{target_module}`."
                    ),
                });
            }
        }
        if let Some(target) = file.parsed.file_call_paths.iter().find_map(|path| {
            helpers::disallowed_sidecar_local_boundary_target(
                path,
                &file.kind,
                owner_module_name,
                local_module_names,
            )
        }) {
            violations.push(RuntimeAssertionsViolation {
                rel_path: file.rel_path.clone(),
                line: None,
                title: "sidecar escapes owned module boundary".to_owned(),
                message: format!(
                    "Internal sidecar harnesses must not escape their owned module boundary through local call path `{target}`."
                ),
            });
        }
        if let Some(target_module) = file.parsed.file_call_paths.iter().find_map(|path| {
            helpers::sibling_module_target(path, owner_module_name, local_module_names)
        }) {
            violations.push(RuntimeAssertionsViolation {
                rel_path: file.rel_path.clone(),
                line: None,
                title: "sidecar calls sibling local module".to_owned(),
                message: format!(
                    "Sidecar file `{}` calls sibling local module `{}`. Call only the owned production module `{}` or the shared assertions crate from this sidecar, so the sidecar tests one module without reaching into siblings.",
                    file.rel_path,
                    target_module,
                    owner_module_name,
                ),
            });
        }
        if let Some(local_root) = file.parsed.file_call_paths.iter().find_map(|path| {
            helpers::first_disallowed_local_package(
                path,
                local_package_names,
                allowed_sidecar_packages,
            )
            .map(str::to_owned)
        }) {
            violations.push(RuntimeAssertionsViolation {
                rel_path: file.rel_path.clone(),
                line: None,
                title: "sidecar calls disallowed local crate".to_owned(),
                message: format!(
                    "Internal sidecar harnesses must not call local crate `{local_root}` directly."
                ),
            });
        }
        if let Some(target_module) = file.parsed.file_call_paths.iter().find_map(|path| {
            helpers::foreign_assertions_module_target(
                path,
                assertions_package_name,
                &file.rel_path,
                owner_module_name,
            )
        }) {
            violations.push(RuntimeAssertionsViolation {
                rel_path: file.rel_path.clone(),
                line: None,
                title: "sidecar calls sibling assertions module".to_owned(),
                message: format!(
                    "Internal sidecar harnesses may only call owned assertions module `{owner_module_name}` and must not call sibling assertions module `{target_module}`."
                ),
            });
        }
    }
}

/// `non_component_harness_violations` function.
fn non_component_harness_violations(
    files: &[G3RsTestAnalyzedSourceFile],
) -> Vec<RuntimeAssertionsViolation> {
    files.iter()
        .filter(|file| file.component_rel_dir.is_none())
        .filter(|file| {
            matches!(
                file.kind,
                G3RsTestFileKind::InternalSidecarMod | G3RsTestFileKind::ExternalHarness
            )
        })
        .map(|file| RuntimeAssertionsViolation {
            rel_path: file.rel_path.clone(),
            line: None,
            title: "test harness outside runtime/assertions split".to_owned(),
            message: "Test harnesses must live under a discovered `runtime` crate with a sibling `assertions` crate; plain root-local sidecars and external harnesses are not allowed.".to_owned(),
        })
        .collect()
}
