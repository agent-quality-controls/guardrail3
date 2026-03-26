use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-05";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(line) = input.function.should_panic_line else {
        return;
    };
    if input.function.should_panic_has_expected {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "should_panic missing expected string".to_owned(),
        message: format!(
            "Test `{}` uses `#[should_panic]` without `expected = \"...\"`.",
            input.function.name
        ),
        file: Some(input.file.rel_path.clone()),
        line: Some(line),
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
#[path = "rs_test_05_should_panic_expected_tests/mod.rs"]
mod rs_test_05_should_panic_expected_tests;
