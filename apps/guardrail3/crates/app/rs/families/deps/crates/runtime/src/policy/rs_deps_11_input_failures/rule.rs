use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::InputFailureDepsInput;

const ID: &str = "RS-DEPS-11";

pub fn check(input: &InputFailureDepsInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "dependency policy input failure".to_owned(),
        input.failure.message.clone(),
        Some(input.failure.rel_path.clone()),
        None,
        false,
    ));
}

