use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::GardeInputFailureInput;

const ID: &str = "RS-GARDE-10";

pub fn check(input: &GardeInputFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "garde-family input failure".to_owned(),
        message: input.failure.message.clone(),
        file: Some(input.failure.rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_garde_10_input_failures_tests/mod.rs"]
mod tests;
