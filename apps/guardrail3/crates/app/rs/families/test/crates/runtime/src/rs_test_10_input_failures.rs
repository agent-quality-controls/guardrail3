use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::InputFailureTestInput;

const ID: &str = "RS-TEST-10";

pub fn check(input: &InputFailureTestInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "test-family input failure".to_owned(),
        message: input.failure.message.clone(),
        file: Some(input.failure.rel_path.clone()),
        line: None,
        inventory: false,
    });
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
#[path = "rs_test_10_input_failures_tests/mod.rs"]
mod rs_test_10_input_failures_tests;