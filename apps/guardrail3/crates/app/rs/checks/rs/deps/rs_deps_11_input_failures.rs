use crate::domain::report::{CheckResult, Severity};

use super::inputs::InputFailureDepsInput;

const ID: &str = "RS-DEPS-11";

pub fn check(input: &InputFailureDepsInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "dependency policy input failure".to_owned(),
        message: input.failure.message.clone(),
        file: Some(input.failure.rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_deps_11_input_failures_tests.rs"]
mod tests;
