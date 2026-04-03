use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ReleaseInputFailureInput;

const ID: &str = "RS-RELEASE-12";

pub fn check(input: &ReleaseInputFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "failed to read release input".to_owned(),
        input.failure.message.clone(),
        Some(input.failure.rel_path.clone()),
        None,
        false,
    ));
}

