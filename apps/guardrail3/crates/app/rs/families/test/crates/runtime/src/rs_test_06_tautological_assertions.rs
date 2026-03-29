use crate::{CheckResult, Severity};

use super::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-06";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    for line in &input.function.tautological_assert_lines {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "tautological assertion".to_owned(),
            message: format!(
                "Test `{}` compares only literals in an assertion and proves nothing.",
                input.function.name
            ),
            file: Some(input.file.rel_path.clone()),
            line: Some(*line),
            inventory: false,
        });
    }
    if input.function.tautological_assert_lines.is_empty() {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "tautological assertions absent".to_owned(),
                message: format!(
                    "Test `{}` uses real values in its assertions.",
                    input.function.name
                ),
                file: Some(input.file.rel_path.clone()),
                line: Some(input.function.line),
                inventory: false,
            }
            .as_inventory(),
        );
    }
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
#[path = "rs_test_06_tautological_assertions_tests/mod.rs"]
mod rs_test_06_tautological_assertions_tests;
