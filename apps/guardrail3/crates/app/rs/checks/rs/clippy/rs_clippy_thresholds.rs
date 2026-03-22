use crate::domain::report::{CheckResult, Severity};

use super::clippy_support::{THRESHOLD_EXPECTATIONS, threshold_value};
use super::inputs::ConfigClippyInput;

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        if let Some(parse_error) = &input.config.parse_error {
            for threshold in THRESHOLD_EXPECTATIONS {
                results.push(CheckResult {
                    id: threshold.id.to_owned(),
                    severity: Severity::Error,
                    title: "clippy.toml parse error".to_owned(),
                    message: format!("Failed to parse clippy.toml: {parse_error}"),
                    file: Some(input.config.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
        }
        return;
    };

    for threshold in THRESHOLD_EXPECTATIONS {
        match threshold_value(parsed, threshold.key) {
            Some(actual) if actual == threshold.expected => results.push(
                CheckResult {
                    id: threshold.id.to_owned(),
                    severity: Severity::Info,
                    title: format!("{} correct", threshold.key),
                    message: format!("{} = {}", threshold.key, threshold.expected),
                    file: Some(input.config.rel_path.clone()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            ),
            Some(actual) => results.push(CheckResult {
                id: threshold.id.to_owned(),
                severity: Severity::Error,
                title: format!("{} wrong value", threshold.key),
                message: format!("Expected {}, got {actual}.", threshold.expected),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            }),
            None => results.push(CheckResult {
                id: threshold.id.to_owned(),
                severity: Severity::Error,
                title: format!("{} missing", threshold.key),
                message: format!("Expected {} = {}.", threshold.key, threshold.expected),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            }),
        }
    }
}
