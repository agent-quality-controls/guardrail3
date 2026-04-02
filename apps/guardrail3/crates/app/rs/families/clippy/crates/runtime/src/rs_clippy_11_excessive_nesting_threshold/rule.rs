use guardrail3_domain_modules::clippy::EXCESSIVE_NESTING_THRESHOLD;
use guardrail3_domain_report::{CheckResult, Severity};

use crate::clippy_support::{IntegerSetting, integer_setting, value_kind};
use crate::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-11";
const KEY: &str = "excessive-nesting-threshold";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    match integer_setting(parsed, KEY) {
        IntegerSetting::Value(actual) if actual == EXCESSIVE_NESTING_THRESHOLD => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                format!("{KEY} correct"),
                format!("{KEY} = {EXCESSIVE_NESTING_THRESHOLD}"),
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
            format!("Expected {EXCESSIVE_NESTING_THRESHOLD}, got {actual}."),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
        IntegerSetting::WrongType(value) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{KEY} wrong type"),
            format!(
                "Expected integer `{KEY} = {EXCESSIVE_NESTING_THRESHOLD}`, found {}.",
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
            format!("Expected {KEY} = {EXCESSIVE_NESTING_THRESHOLD}."),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
    }
}



// reason: test-only sidecar module wiring
