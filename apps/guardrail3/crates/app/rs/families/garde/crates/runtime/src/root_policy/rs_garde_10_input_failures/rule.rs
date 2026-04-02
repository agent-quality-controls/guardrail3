use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::GardeInputFailureInput;

const ID: &str = "RS-GARDE-10";

pub fn check(input: &GardeInputFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "garde-family input failure".to_owned(),
        input.failure.message.clone(),
        Some(input.failure.rel_path.clone()),
        None,
        false,
    ));
}

