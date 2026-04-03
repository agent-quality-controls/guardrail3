use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::CargoConfigOverrideInput;

const ID: &str = "RS-CLIPPY-24";

pub fn check_clean(results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "no clippy config dir overrides found".to_owned(),
            "No applicable cargo config surfaces set `CLIPPY_CONF_DIR`.".to_owned(),
            None,
            None,
            false,
        )
        .as_inventory(),
    );
}

pub fn check(input: &CargoConfigOverrideInput<'_>, results: &mut Vec<CheckResult>) {
    let (title, message) = match input.parse_error {
        Some(parse_error) => (
            "cargo config override surface is not parseable".to_owned(),
            format!(
                "Failed to parse `{}` while checking for forbidden `CLIPPY_CONF_DIR` overrides: {parse_error}",
                input.rel_path
            ),
        ),
        None => (
            "clippy config dir override is forbidden".to_owned(),
            format!(
                "`{}` sets `CLIPPY_CONF_DIR`, which bypasses the routed clippy policy-root model. Remove the `CLIPPY_CONF_DIR` setting from `{}`.",
                input.rel_path, input.rel_path
            ),
        ),
    };

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title,
        message,
        file: Some(input.rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}





// reason: test-only sidecar module wiring
