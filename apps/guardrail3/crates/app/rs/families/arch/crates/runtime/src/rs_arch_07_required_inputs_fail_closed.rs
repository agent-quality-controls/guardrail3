use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RequiredInputFailureInput;

const ID: &str = "RS-ARCH-07";

pub fn check(input: &RequiredInputFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "Rust architecture required input failed closed".to_owned(),
        message: input.failure.message.clone(),
        file: Some(input.failure.rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_arch_07_required_inputs_fail_closed_tests/mod.rs"]
mod rs_arch_07_required_inputs_fail_closed_tests;
