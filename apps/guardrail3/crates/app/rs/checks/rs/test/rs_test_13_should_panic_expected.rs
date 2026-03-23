use crate::domain::report::{CheckResult, Severity};

use super::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-13";

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
        title: "should_panic missing expected message".to_owned(),
        message: format!("Test `{}` uses `#[should_panic]` without `expected = ...`.", input.function.name),
        file: Some(input.file.rel_path.clone()),
        line: Some(line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_test_13_should_panic_expected_tests.rs"]
mod tests;
