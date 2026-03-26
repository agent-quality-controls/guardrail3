use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::InputFailureCargoInput;

const ID: &str = "RS-CARGO-14";

pub fn check(input: &InputFailureCargoInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "cargo-family input failure".to_owned(),
        message: input.failure.message.clone(),
        file: Some(input.failure.rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_cargo_14_input_failures_tests/mod.rs"]
mod rs_cargo_14_input_failures_tests;
