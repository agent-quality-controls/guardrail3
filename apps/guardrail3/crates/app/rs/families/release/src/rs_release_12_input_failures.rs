use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ReleaseInputFailureInput;

const ID: &str = "RS-RELEASE-12";

pub fn check(input: &ReleaseInputFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "Release-family input failure".to_owned(),
        message: input.failure.message.clone(),
        file: Some(input.failure.rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_release_12_input_failures_tests/mod.rs"]
mod tests;
