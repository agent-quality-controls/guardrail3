#![expect(
    clippy::too_many_lines,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use std::collections::BTreeSet;

use g3rs_test_types::{G3RsTestComponentFileTreeFacts, G3RsTestFileKind};

use super::helpers;
use super::violations::RuntimeAssertionsViolation;

/// `collect_assertions_module_violations` function.
pub(super) fn collect_assertions_module_violations(
    violations: &mut Vec<RuntimeAssertionsViolation>,
    component: &G3RsTestComponentFileTreeFacts,
    local_package_names: &BTreeSet<String>,
    allowed_assertions_packages: &BTreeSet<String>,
) {
    for file in component
        .assertions_module_files
        .iter()
        .filter(|file| matches!(file.kind, G3RsTestFileKind::AssertionsModule))
    {
        for binding in &file.parsed.imports {
            if helpers::import_uses_local_boundary(binding)
                && !is_public_assertion_reexport(binding, &file.proof_bearing_assertion_functions)
            {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.rel_path.clone(),
                    line: Some(binding.line),
                    title: "assertions module reaches local private code".to_owned(),
                    message: format!(
                        "Assertions file `{}` imports local path `{}`. Import the runtime crate public API instead, so sidecars and external harnesses can reuse the same assertions without depending on private module layout.",
                        file.rel_path,
                        binding.path_segments.join("::"),
                    ),
                });
            }
            if let Some(local_root) = helpers::first_disallowed_local_package(
                &binding.path_segments,
                local_package_names,
                allowed_assertions_packages,
            ) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.rel_path.clone(),
                    line: Some(binding.line),
                    title: "assertions module imports disallowed local crate".to_owned(),
                    message: format!(
                        "Assertions modules must not import local crate `{local_root}` directly."
                    ),
                });
            }
            if path_mentions_route_construction(&binding.path_segments) {
                violations.push(RuntimeAssertionsViolation {
                    rel_path: file.rel_path.clone(),
                    line: Some(binding.line),
                    title: "assertions module imports route construction infrastructure".to_owned(),
                    message: "Assertions modules must stay reusable semantic proof helpers and must not import route-construction infrastructure.".to_owned(),
                });
            }
        }
        if let Some(local_root) = file.parsed.file_call_paths.iter().find_map(|path| {
            helpers::first_disallowed_local_package(
                path,
                local_package_names,
                allowed_assertions_packages,
            )
            .map(str::to_owned)
        }) {
            violations.push(RuntimeAssertionsViolation {
                rel_path: file.rel_path.clone(),
                line: None,
                title: "assertions module calls disallowed local crate".to_owned(),
                message: format!(
                    "Assertions modules must not call local crate `{local_root}` directly."
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
                .flat_map(|function| function.body.path_uses.iter())
                .any(|path| path_mentions_route_construction(path))
        {
            violations.push(RuntimeAssertionsViolation {
                rel_path: file.rel_path.clone(),
                line: None,
                title: "assertions module builds routed family input".to_owned(),
                message: "Assertions modules must stay reusable semantic proof helpers and must not construct routed family inputs through mapper/placement wiring.".to_owned(),
            });
        }
        if helpers::assertions_call_runtime_check_test_tree(
            &file.parsed.imports,
            &file.parsed.file_call_paths,
            &file.parsed.functions,
            component.runtime_package_name.as_deref(),
        ) {
            violations.push(RuntimeAssertionsViolation {
                rel_path: file.rel_path.clone(),
                line: None,
                title: "assertions module orchestrates family execution".to_owned(),
                message: "Assertions modules must not call runtime `check_test_tree(...)`; sidecars own family execution and assertions own reusable semantic proof only.".to_owned(),
            });
        }
    }
}

/// `is_public_assertion_reexport` function.
fn is_public_assertion_reexport(
    binding: &g3rs_test_types::ast::UseBinding,
    proof_bearing_assertion_functions: &BTreeSet<String>,
) -> bool {
    binding.is_public
        && public_reexport_target_path(&binding.path_segments)
            .is_some_and(|target| proof_bearing_assertion_functions.contains(&target))
}

/// `public_reexport_target_path` function.
fn public_reexport_target_path(path_segments: &[String]) -> Option<String> {
    let [root, rest @ ..] = path_segments else {
        return None;
    };
    match root.as_str() {
        "crate" => Some(rest.join("::")),
        _ => None,
    }
}

/// `path_mentions_route_construction` function.
fn path_mentions_route_construction(path: &[String]) -> bool {
    path.iter().any(|segment| {
        matches!(
            segment.as_str(),
            "FamilyMapper" | "guardrail3_app_rs_placement"
        )
    })
}
