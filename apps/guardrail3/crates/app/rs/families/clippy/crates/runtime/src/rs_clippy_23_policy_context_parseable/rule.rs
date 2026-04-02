use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PolicyContextFailureInput;

const ID: &str = "RS-CLIPPY-23";

pub fn check_parseable(results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "clippy policy context parseable".to_owned(),
            "Active `guardrail3.toml` parsed successfully for clippy policy context.".to_owned(),
            Some("guardrail3.toml".to_owned()),
            None,
            false,
        )
        .as_inventory(),
    );
}

pub fn check(input: &PolicyContextFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "clippy policy context is not parseable".to_owned(),
        format!(
            "Failed to parse active `guardrail3.toml` used for clippy profile and garde policy: {}",
            input.parse_error
        ),
        Some("guardrail3.toml".to_owned()),
        None,
        false,
    ));
}



// reason: test-only sidecar module wiring
