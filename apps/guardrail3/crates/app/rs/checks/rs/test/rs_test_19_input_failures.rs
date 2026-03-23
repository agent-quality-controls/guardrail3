use crate::domain::report::{CheckResult, Severity};

use super::inputs::InputFailureTestInput;

const ID: &str = "RS-TEST-19";

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
#[path = "rs_test_19_input_failures_tests.rs"]
mod tests;
