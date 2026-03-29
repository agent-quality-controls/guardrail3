use crate::{CheckResult, Severity};

use super::facts::TestFileKind;
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
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "assertions module lacks proof-bearing export".to_owned(),
        message: "Assertions modules that expose helper functions must contain at least one public function with a real assertion or a call into another proof-bearing owned assertions function.".to_owned(),
        file: Some(input.file.rel_path.clone()),
        line: Some(first_exported_function.line),
        inventory: false,
    });
}

pub fn check_sidecar_semantic_proof(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    if !matches!(input.file.kind, TestFileKind::InternalSidecarSupport)
        && !matches!(input.file.kind, TestFileKind::InternalSidecarMod)
    {
        return;
    }
    if input.file.assertions_package_name.is_none() || !input.function.has_failure_enforcement {
        return;
    }
    if !owns_direct_result_shape_assertion(input) {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "sidecar owns semantic result assertion".to_owned(),
        message: "Internal sidecars must keep scenario setup local but push guardrail result assertions into the owned sibling assertions module.".to_owned(),
        file: Some(input.file.rel_path.clone()),
        line: Some(input.function.line),
        inventory: false,
    });
}

fn owns_direct_result_shape_assertion(input: &TestFunctionInput<'_>) -> bool {
    input
        .function
        .string_literals
        .iter()
        .any(|value| value.starts_with("RS-"))
        || input
            .function
            .field_accesses
            .iter()
            .any(|field| REPORT_FIELDS.contains(&field.name.as_str()))
        || input.function.path_uses.iter().any(|path| {
            path.last()
                .is_some_and(|segment| matches!(segment.as_str(), "CheckResult" | "Severity"))
        })
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
#[path = "rs_test_16_assertions_modules_prove_tests/mod.rs"]
mod rs_test_16_assertions_modules_prove_tests;
