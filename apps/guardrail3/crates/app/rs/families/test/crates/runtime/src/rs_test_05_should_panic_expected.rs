use crate::{CheckResult, Severity};

use super::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-05";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(line) = input.function.should_panic_line else {
        return;
    };
    if input.function.should_panic_has_expected {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "should_panic expected string present".to_owned(),
                message: format!(
                    "Test `{}` keeps `#[should_panic]` paired with an explicit expected string.",
                    input.function.name
                ),
                file: Some(input.file.rel_path.clone()),
                line: Some(line),
                inventory: false,
            }
            .as_inventory(),
        );
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
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    super::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]
#[path = "rs_test_05_should_panic_expected_tests/mod.rs"]
mod rs_test_05_should_panic_expected_tests;
