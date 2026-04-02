use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

mod helpers;

use crate::analysis::AnalyzedFile;
use crate::{CheckResult, Severity};

use crate::facts::{RuntimeAssertionsViolation, TestFileKind, TestRootFacts};
use crate::inputs::RuntimeAssertionsViolationInput;

const ID: &str = "RS-TEST-03";

fn path_mentions_route_construction(path: &[String]) -> bool {
    path.iter().any(|segment| {
        matches!(
            segment.as_str(),
            "FamilyMapper" | "guardrail3_app_rs_placement"
        )
    })
}

pub(crate) fn collect(
    tree: &ProjectTree,
    root: &TestRootFacts,
    files: &[AnalyzedFile],
    scoped_files: Option<&BTreeSet<String>>,
    local_package_names: &BTreeSet<String>,
    results: &mut Vec<CheckResult>,
) {
    let violations = collect_violations(tree, root, files, scoped_files, local_package_names);
    if violations.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "runtime/assertions split confirmed".to_owned(),
                format!(
                    "Root `{}` keeps runtime harnesses separated from sibling assertions crates.",
                    root.rel_dir
                ),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
    for violation in violations {
        check(&RuntimeAssertionsViolationInput::new(&violation), results);
    }
}

pub fn check(input: &RuntimeAssertionsViolationInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        input.violation.title.clone(),
        input.violation.message.clone(),
        Some(input.violation.rel_path.clone()),
        input.violation.line,
        false,
    ));
}

fn collect_violations(
    tree: &ProjectTree,
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
        let allowed_external_packages = helpers::allowed_external_harness_packages(component);
        let allowed_sidecar_packages = helpers::allowed_sidecar_packages(component);
        let allowed_assertions_packages = helpers::allowed_assertions_packages(component);
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
            if !tree.file_exists(&sidecar.assertions_module_rel_path) {
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
                if let Some(target) = helpers::disallowed_sidecar_local_boundary_target(
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
                if helpers::import_hits_sibling_module(
                    binding,
                    owner_module_name,
                    &local_module_names,
                ) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.facts.rel_path.clone(),
                        line: Some(binding.line),
                        title: "sidecar imports sibling production module".to_owned(),
                        message: "Internal sidecar harnesses may reach only their owned production module subtree, not sibling production modules.".to_owned(),
                    });
                }
                if let Some(local_root) = helpers::first_disallowed_local_package(
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
                if let Some(target_module) = helpers::foreign_assertions_module_target(
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
                helpers::disallowed_sidecar_local_boundary_target(
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
                helpers::sibling_module_target(path, owner_module_name, &local_module_names)
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
                helpers::first_disallowed_local_package(
                    path,
                    local_package_names,
                    &allowed_sidecar_packages,
                )
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
                helpers::foreign_assertions_module_target(
                    path,
                    assertions_package_name,
                    owner_module_name,
                )
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
                if helpers::import_uses_local_boundary(binding) {
                    violations.push(RuntimeAssertionsViolation {
                        rel_path: file.facts.rel_path.clone(),
                        line: Some(binding.line),
                        title: "assertions module reaches local private code".to_owned(),
                        message: "Assertions modules must import runtime public API or helper crates, not local `crate::`, `self::`, or `super::` paths.".to_owned(),
                    });
                }
                if let Some(local_root) = helpers::first_disallowed_local_package(
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
                if path_mentions_route_construction(&binding.path_segments) {
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
                helpers::first_disallowed_local_package(
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
                .any(|call_path| path_mentions_route_construction(call_path))
                || file
                    .parsed
                    .functions
                    .iter()
                    .flat_map(|function| function.path_uses.iter())
                    .any(|path| path_mentions_route_construction(path))
            {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.facts.rel_path.clone(),
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

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    crate::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
