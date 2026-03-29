use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::{InputFailureCargoInput, InputFailureInventoryCargoInput};

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

pub fn check_inventory(
    input: &InputFailureInventoryCargoInput<'_>,
    results: &mut Vec<CheckResult>,
) {
    if input.has_input_failures {
        return;
    }

    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "cargo-family inputs parsed cleanly".to_owned(),
            message: "Active Cargo policy inputs parsed without cargo-family input failures."
                .to_owned(),
            file: Some(input.root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

#[cfg(test)]
#[path = "rs_cargo_14_input_failures_tests/mod.rs"]
mod rs_cargo_14_input_failures_tests;
