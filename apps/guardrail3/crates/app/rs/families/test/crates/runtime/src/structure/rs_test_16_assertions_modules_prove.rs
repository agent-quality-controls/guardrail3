use crate::{CheckResult, Severity};

use super::facts::TestFileKind;
use super::parse::{FieldAccessInfo, FunctionInfo};
use super::inputs::{AssertionsModuleInput, TestFunctionInput};

const ID: &str = "RS-TEST-16";
const REPORT_FIELDS: &[&str] = &[
    "file",
    "id",
    "inventory",
    "line",
    "message",
    "severity",
    "title",
];
const REPORT_METHODS: &[&str] = &[
    "file",
    "id",
    "inventory",
    "line",
    "message",
    "severity",
    "title",
];

pub fn check(input: &AssertionsModuleInput<'_>, results: &mut Vec<CheckResult>) {
    let first_exported_function = input
        .parsed
        .functions
        .iter()
        .find(|function| function.is_public && !function.is_test);
    let Some(first_exported_function) = first_exported_function else {
        return;
    };
    if !input.proof_bearing_exported_functions.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "assertions module proves runtime".to_owned(),
                "Assertions modules expose public helpers that ultimately prove through real assertions.".to_owned(),
                Some(input.file.rel_path.clone()),
                Some(first_exported_function.line),
                false,
            )
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    "assertions module lacks proof-bearing export".to_owned(),
    "Assertions modules that expose helper functions must contain at least one public function with a real assertion or a call into another proof-bearing owned assertions function.".to_owned(),
    Some(input.file.rel_path.clone()),
    Some(first_exported_function.line),
    false,
    ));
}

pub fn check_sidecar_semantic_proof(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    if !matches!(input.file.kind, TestFileKind::InternalSidecarSupport)
        && !matches!(input.file.kind, TestFileKind::InternalSidecarMod)
    {
        return;
    }
    if input.file.assertions_package_name.is_none() {
        return;
    }
    if !owns_sidecar_semantic_proof(input) {
        if !input.function.has_failure_enforcement {
            return;
        }
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "sidecar delegates semantic proof".to_owned(),
                "Internal sidecars keep setup local and delegate result-shape assertions to the owned sibling assertions module.".to_owned(),
                Some(input.file.rel_path.clone()),
                Some(input.function.line),
                false,
            )
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    "sidecar owns semantic result assertion".to_owned(),
    "Internal sidecars must keep scenario setup local but push guardrail result assertions into the owned sibling assertions module.".to_owned(),
    Some(input.file.rel_path.clone()),
    Some(input.function.line),
    false,
    ));
}

fn owns_sidecar_semantic_proof(input: &TestFunctionInput<'_>) -> bool {
    owns_result_shape_assertion(
        &input.function.field_accesses,
        &input.function.method_names,
        &input.function.path_uses,
    ) || local_semantic_helper_names(&input.parsed.functions)
        .iter()
        .any(|helper| {
            input.function.call_paths.iter().any(|path| {
                path.len() == 1
                    && path[0] == *helper
                    && !input.function.shadowed_idents.contains(&path[0])
            })
        })
}

fn local_semantic_helper_names<'a>(functions: &'a [FunctionInfo]) -> std::collections::BTreeSet<&'a str> {
    let mut semantic_helpers = functions
        .iter()
        .filter(|function| !function.is_test)
        .filter(|function| {
            owns_result_shape_assertion(
                &function.field_accesses,
                &function.method_names,
                &function.path_uses,
            )
        })
        .map(|function| function.name.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    loop {
        let mut changed = false;
        for function in functions.iter().filter(|function| !function.is_test) {
            if semantic_helpers.contains(function.name.as_str()) {
                continue;
            }
            if function.call_paths.iter().any(|path| {
                path.len() == 1
                    && semantic_helpers.contains(path[0].as_str())
                    && !function.shadowed_idents.contains(&path[0])
            }) {
                changed |= semantic_helpers.insert(function.name.as_str());
            }
        }
        if !changed {
            break;
        }
    }

    semantic_helpers
}

fn owns_result_shape_assertion(
    field_accesses: &[FieldAccessInfo],
    method_names: &[String],
    path_uses: &[Vec<String>],
) -> bool {
    field_accesses
        .iter()
        .any(|field| REPORT_FIELDS.contains(&field.name.as_str()))
        || method_names
            .iter()
            .any(|method| REPORT_METHODS.contains(&method.as_str()))
        || path_uses.iter().any(|path| {
            path.last()
                .is_some_and(|segment| matches!(segment.as_str(), "CheckResult" | "Severity"))
        })
}


#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    super::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]
#[path = "rs_test_16_assertions_modules_prove_tests/mod.rs"]
mod rs_test_16_assertions_modules_prove_tests;
