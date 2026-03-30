use crate::{CheckResult, Severity};

use super::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-05";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(line) = input.function.should_panic_line else {
        return;
    };
    if input.function.should_panic_has_expected {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "should_panic expected string present".to_owned(),
                format!(
                    "Test `{}` keeps `#[should_panic]` paired with an explicit expected string.",
                    input.function.name
                ),
                Some(input.file.rel_path.clone()),
                Some(line),
                false,
            )
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Warn,
    "should_panic missing expected string".to_owned(),
    format!(
            "Test `{}` uses `#[should_panic]` without `expected = \"...\"`.",
            input.function.name
        ),
    Some(input.file.rel_path.clone()),
    Some(line),
    false,
    ));
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    super::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]
#[path = "rs_test_05_should_panic_expected_tests/mod.rs"]
mod rs_test_05_should_panic_expected_tests;
