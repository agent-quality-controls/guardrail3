use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::{InputFailureCargoInput, InputFailureInventoryCargoInput};

const ID: &str = "RS-CARGO-14";

pub fn check(input: &InputFailureCargoInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "cargo-family input failure".to_owned(),
        input.failure.message.clone(),
        Some(input.failure.rel_path.clone()),
        None,
        false,
    ));
}

pub fn check_inventory(
    input: &InputFailureInventoryCargoInput<'_>,
    results: &mut Vec<CheckResult>,
) {
    if input.has_input_failures {
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "cargo-family inputs parsed cleanly".to_owned(),
            "Active Cargo policy inputs parsed without cargo-family input failures.".to_owned(),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}

#[cfg(test)]
#[path = "rs_cargo_14_input_failures_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_cargo_14_input_failures_tests;
