use std::collections::{BTreeMap, BTreeSet};

use g3rs_test_file_tree_checks_types::G3RsTestFileTreeChecksInput;
use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use super::helpers;
use crate::support::{AnalyzedFile, RootAnalysis};

const ID: &str = "RS-TEST-FILETREE-03";

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeAssertionsViolation {
    rel_path: String,
    line: Option<usize>,
    title: String,
    message: String,
}

fn component_package_rel_dir(component: &g3rs_test_types::G3RsTestComponentFileTreeFacts) -> &str {
    if component.rel_dir.is_empty() {
        component.runtime_rel_dir.as_str()
    } else {
        component.rel_dir.as_str()
    }
}

fn path_mentions_route_construction(path: &[String]) -> bool {
    path.iter().any(|segment| {
        matches!(
            segment.as_str(),
            "FamilyMapper" | "guardrail3_app_rs_placement"
        )
    })
}

pub(crate) fn collect(
    input: &G3RsTestFileTreeChecksInput,
    analysis: &RootAnalysis,
    results: &mut Vec<G3CheckResult>,
) {
    let violations = collect_violations(input, analysis);
    if violations.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "runtime/assertions split confirmed".to_owned(),
                format!(
                    "Root `{}` keeps runtime harnesses separated from sibling assertions crates.",
                    input.root_rel_dir
                ),
                Some(input.cargo_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    for violation in violations {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            violation.title,
            violation.message,
            Some(violation.rel_path),
            violation.line,
        ));
    }
}

fn collect_violations(
    input: &G3RsTestFileTreeChecksInput,
    analysis: &RootAnalysis,
) -> Vec<RuntimeAssertionsViolation> {
    let mut violations = Vec::new();
    let parsed_by_path = analysis
        .files
        .iter()
        .map(|file| (file.file.rel_path.clone(), file))
        .collect::<BTreeMap<_, _>>();

    violations.extend(non_component_harness_violations(&analysis.files));

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
            if !parsed_by_path.contains_key(&sidecar.assertions_module_rel_path) {
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

        for external_harness in &component.external_harnesses {
            let Some(file) = parsed_by_path.get(external_harness) else {
                continue;
            };

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
                    &input.local_package_names,
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
                    &input.local_package_names,
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

        for file in analysis.files.iter().filter(|file| {
            file.file.component_rel_dir.as_deref() == Some(component.rel_dir.as_str())
                && matches!(
                    file.file.kind,
                    G3RsTestFileKind::InternalSidecarMod | G3RsTestFileKind::InternalSidecarSupport
                )
        }) {
            let Some(owner_module_name) = file.file.owner_module_name.as_deref() else {
                continue;
            };
            let local_module_names = analysis
                .files
                .iter()
                .filter(|candidate| {
                    candidate.file.component_rel_dir.as_deref() == Some(component.rel_dir.as_str())
                        && matches!(candidate.file.kind, G3RsTestFileKind::Source)
                })
                .filter_map(|candidate| candidate.file.owner_module_name.clone())
                .collect::<BTreeSet<_>>();
            for binding in &file.parsed.imports {
                if let Some(target) = helpers::disallowed_sidecar_local_boundary_target(
                    &binding.path_segments,
                    &file.file.kind,
                    owner_module_name,
                    &local_module_names,
                ) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.file.rel_path.clone(),
                        line: Some(binding.line),
                        title: "sidecar escapes owned module boundary".to_owned(),
                        message: format!(
                            "Sidecar file `{}` reaches local path `{}`. Call only the owned production module `{}` or the shared assertions crate from this sidecar, so the sidecar tests one module without reaching into siblings.",
                            file.file.rel_path,
                            target,
                            owner_module_name,
                        ),
                    });
                }
                if helpers::import_hits_sibling_module(
                    binding,
                    owner_module_name,
                    &local_module_names,
                ) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.file.rel_path.clone(),
                        line: Some(binding.line),
                        title: "sidecar imports sibling local module".to_owned(),
                        message: format!(
                            "Sidecar file `{}` imports sibling local module `{}`. Import only the owned production module `{}` or the shared assertions crate from this sidecar, so the sidecar tests one module without reaching into siblings.",
                            file.file.rel_path,
                            helpers::sibling_module_target(
                                &binding.path_segments,
                                owner_module_name,
                                &local_module_names,
                            )
                            .unwrap_or("<sibling-module>"),
                            owner_module_name,
                        ),
                    });
                }
                if let Some(local_root) = helpers::first_disallowed_local_package(
                    &binding.path_segments,
                    &input.local_package_names,
                    &allowed_sidecar_packages,
                ) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.file.rel_path.clone(),
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
                    &file.file.rel_path,
                    owner_module_name,
                ) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.file.rel_path.clone(),
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
                    &file.file.kind,
                    owner_module_name,
                    &local_module_names,
                )
            }) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.file.rel_path.clone(),
                    line: None,
                    title: "sidecar escapes owned module boundary".to_owned(),
                    message: format!(
                        "Internal sidecar harnesses must not escape their owned module boundary through local call path `{target}`."
                    ),
                });
            }
            if let Some(target_module) = file.parsed.file_call_paths.iter().find_map(|path| {
                helpers::sibling_module_target(path, owner_module_name, &local_module_names)
            }) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.file.rel_path.clone(),
                    line: None,
                    title: "sidecar calls sibling local module".to_owned(),
                    message: format!(
                        "Sidecar file `{}` calls sibling local module `{}`. Call only the owned production module `{}` or the shared assertions crate from this sidecar, so the sidecar tests one module without reaching into siblings.",
                        file.file.rel_path,
                        target_module,
                        owner_module_name,
                    ),
                });
            }
            if let Some(local_root) = file.parsed.file_call_paths.iter().find_map(|path| {
                helpers::first_disallowed_local_package(
                    path,
                    &input.local_package_names,
                    &allowed_sidecar_packages,
                )
                .map(str::to_owned)
            }) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.file.rel_path.clone(),
                    line: None,
                    title: "sidecar calls disallowed local crate".to_owned(),
                    message: format!(
                        "Internal sidecar harnesses must not call local crate `{}` directly.",
                        local_root
                    ),
                });
            }
            if let Some(target_module) = file.parsed.file_call_paths.iter().find_map(|path| {
                helpers::foreign_assertions_module_target(
                    path,
                    assertions_package_name,
                    &file.file.rel_path,
                    owner_module_name,
                )
            }) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.file.rel_path.clone(),
                    line: None,
                    title: "sidecar calls sibling assertions module".to_owned(),
                    message: format!(
                        "Internal sidecar harnesses may only call owned assertions module `{owner_module_name}` and must not call sibling assertions module `{target_module}`."
                    ),
                });
            }
        }

        for file in analysis.files.iter().filter(|file| {
            file.file.component_rel_dir.as_deref() == Some(component.rel_dir.as_str())
                && matches!(file.file.kind, G3RsTestFileKind::AssertionsModule)
        }) {
            for binding in &file.parsed.imports {
                if helpers::import_uses_local_boundary(binding) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.file.rel_path.clone(),
                        line: Some(binding.line),
                        title: "assertions module reaches local private code".to_owned(),
                        message: format!(
                            "Assertions file `{}` imports local path `{}`. Import the runtime crate public API instead, so sidecars and external harnesses can reuse the same assertions without depending on private module layout.",
                            file.file.rel_path,
                            binding.path_segments.join("::"),
                        ),
                    });
                }
                if let Some(local_root) = helpers::first_disallowed_local_package(
                    &binding.path_segments,
                    &input.local_package_names,
                    &allowed_assertions_packages,
                ) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.file.rel_path.clone(),
                        line: Some(binding.line),
                        title: "assertions module imports disallowed local crate".to_owned(),
                        message: format!(
                            "Assertions modules must not import local crate `{local_root}` directly."
                        ),
                    });
                }
                if path_mentions_route_construction(&binding.path_segments) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.file.rel_path.clone(),
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
                helpers::first_disallowed_local_package(
                    path,
                    &input.local_package_names,
                    &allowed_assertions_packages,
                )
                .map(str::to_owned)
            }) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.file.rel_path.clone(),
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
                .any(|call_path| path_mentions_route_construction(call_path))
                || file
                    .parsed
                    .functions
                    .iter()
                    .flat_map(|function| function.path_uses.iter())
                    .any(|path| path_mentions_route_construction(path))
            {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.file.rel_path.clone(),
                    line: None,
                    title: "assertions module builds routed family input".to_owned(),
                    message: "Assertions modules must stay reusable semantic proof helpers and must not construct routed family inputs through mapper/placement wiring.".to_owned(),
                });
            }
            if helpers::assertions_call_runtime_check_test_tree(
                &file.parsed.imports,
                &file.parsed.file_call_paths,
                component.runtime_package_name.as_deref(),
            ) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.file.rel_path.clone(),
                    line: None,
                    title: "assertions module orchestrates family execution".to_owned(),
                    message: "Assertions modules must not call runtime `check_test_tree(...)`; sidecars own family execution and assertions own reusable semantic proof only.".to_owned(),
                });
            }
        }
    }

    violations.sort_by(|left, right| {
        left.rel_path
            .cmp(&right.rel_path)
            .then_with(|| left.line.cmp(&right.line))
            .then_with(|| left.title.cmp(&right.title))
    });
    violations
}

fn non_component_harness_violations(files: &[AnalyzedFile]) -> Vec<RuntimeAssertionsViolation> {
    files.iter()
        .filter(|file| file.file.component_rel_dir.is_none())
        .filter(|file| {
            matches!(
                file.file.kind,
                G3RsTestFileKind::InternalSidecarMod | G3RsTestFileKind::ExternalHarness
            )
        })
        .map(|file| RuntimeAssertionsViolation {
            rel_path: file.file.rel_path.clone(),
            line: None,
            title: "test harness outside runtime/assertions split".to_owned(),
            message: "Test harnesses must live under a discovered `runtime` crate with a sibling `assertions` crate; plain root-local sidecars and external harnesses are not allowed.".to_owned(),
        })
        .collect()
}
