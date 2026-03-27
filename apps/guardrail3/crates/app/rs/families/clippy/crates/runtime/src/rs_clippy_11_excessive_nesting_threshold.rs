use guardrail3_domain_modules::clippy::EXCESSIVE_NESTING_THRESHOLD;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::threshold_value;
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-11";
const KEY: &str = "excessive-nesting-threshold";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        if let Some(parse_error) = &input.config.parse_error {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "clippy.toml parse error".to_owned(),
                message: format!("Failed to parse clippy.toml: {parse_error}"),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
        return;
    };

    match threshold_value(parsed, KEY) {
        Some(actual) if actual == EXCESSIVE_NESTING_THRESHOLD => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: format!("{KEY} correct"),
                message: format!("{KEY} = {EXCESSIVE_NESTING_THRESHOLD}"),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        Some(actual) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("{KEY} wrong value"),
            message: format!("Expected {EXCESSIVE_NESTING_THRESHOLD}, got {actual}."),
            file: Some(input.config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("{KEY} missing"),
            message: format!("Expected {KEY} = {EXCESSIVE_NESTING_THRESHOLD}."),
            file: Some(input.config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
#[path = "rs_clippy_11_excessive_nesting_threshold_tests/mod.rs"]
mod rs_clippy_11_excessive_nesting_threshold_tests;
