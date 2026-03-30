use guardrail3_domain_modules::clippy::TOO_MANY_LINES_THRESHOLD;
#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{IntegerSetting, integer_setting, value_kind};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-09";
const KEY: &str = "too-many-lines-threshold";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    match integer_setting(parsed, KEY) {
        IntegerSetting::Value(actual) if actual == TOO_MANY_LINES_THRESHOLD => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: format!("{KEY} correct"),
                message: format!("{KEY} = {TOO_MANY_LINES_THRESHOLD}"),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        IntegerSetting::Value(actual) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("{KEY} wrong value"),
            message: format!("Expected {TOO_MANY_LINES_THRESHOLD}, got {actual}."),
            file: Some(input.config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        IntegerSetting::WrongType(value) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("{KEY} wrong type"),
            message: format!(
                "Expected integer `{KEY} = {TOO_MANY_LINES_THRESHOLD}`, found {}.",
                value_kind(value)
            ),
            file: Some(input.config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        IntegerSetting::Missing => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("{KEY} missing"),
            message: format!("Expected {KEY} = {TOO_MANY_LINES_THRESHOLD}."),
            file: Some(input.config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
pub(crate) fn run_for_tests(tree: &ProjectTree, rel_path: &str) -> Vec<CheckResult> {
    let facts = super::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    check(
        &super::facts::config_input_for_tests(&facts, rel_path),
        &mut results,
    );
    results
}

#[cfg(test)]
#[path = "rs_clippy_09_too_many_lines_threshold_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_clippy_09_too_many_lines_threshold_tests;
