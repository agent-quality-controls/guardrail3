use guardrail3_domain_modules::clippy::TYPE_COMPLEXITY_THRESHOLD;
#[cfg(test)]
use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{IntegerSetting, integer_setting, value_kind};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-22";
const KEY: &str = "type-complexity-threshold";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    match integer_setting(parsed, KEY) {
        IntegerSetting::Value(actual) if actual == TYPE_COMPLEXITY_THRESHOLD => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                format!("{KEY} correct"),
                format!("{KEY} = {TYPE_COMPLEXITY_THRESHOLD}"),
                Some(input.config.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
        IntegerSetting::Value(actual) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{KEY} wrong value"),
            format!("Expected {TYPE_COMPLEXITY_THRESHOLD}, got {actual}."),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
        IntegerSetting::WrongType(value) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{KEY} wrong type"),
            format!(
                "Expected integer `{KEY} = {TYPE_COMPLEXITY_THRESHOLD}`, found {}.",
                value_kind(value)
            ),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
        IntegerSetting::Missing => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{KEY} missing"),
            format!("Expected {KEY} = {TYPE_COMPLEXITY_THRESHOLD}."),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
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
#[path = "rs_clippy_22_type_complexity_threshold_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_clippy_22_type_complexity_threshold_tests;
