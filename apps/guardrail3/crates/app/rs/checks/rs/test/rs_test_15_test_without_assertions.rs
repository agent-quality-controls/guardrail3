use crate::domain::report::{CheckResult, Severity};

use super::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-15";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    if input.function.has_result_return
        || input.function.has_assertion_macro
        || input.function.has_assert_like_call
    {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "test has no assertions".to_owned(),
        message: format!(
            "Test `{}` has no assertion macro or assertion-like call.",
            input.function.name
        ),
        file: Some(input.file.rel_path.clone()),
        line: Some(input.function.line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_test_15_test_without_assertions_tests.rs"]
mod tests;
