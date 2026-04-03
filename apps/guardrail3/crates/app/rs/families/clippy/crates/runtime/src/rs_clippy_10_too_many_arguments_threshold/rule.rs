use guardrail3_domain_modules::clippy::TOO_MANY_ARGUMENTS_THRESHOLD;
use guardrail3_domain_report::{CheckResult, Severity};

use crate::clippy_support::{IntegerSetting, integer_setting, value_kind};
use crate::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-10";
const KEY: &str = "too-many-arguments-threshold";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    match integer_setting(parsed, KEY) {
        IntegerSetting::Value(actual) if actual == TOO_MANY_ARGUMENTS_THRESHOLD => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                format!("{KEY} correct"),
                format!("{KEY} = {TOO_MANY_ARGUMENTS_THRESHOLD}"),
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
            format!("Expected {TOO_MANY_ARGUMENTS_THRESHOLD}, got {actual}. Set `{KEY} = {TOO_MANY_ARGUMENTS_THRESHOLD}` in clippy.toml."),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
        IntegerSetting::WrongType(value) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{KEY} wrong type"),
            format!(
                "Expected integer `{KEY} = {TOO_MANY_ARGUMENTS_THRESHOLD}`, found {}.",
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
            format!("Add `{KEY} = {TOO_MANY_ARGUMENTS_THRESHOLD}` to clippy.toml."),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
    }
}



// reason: test-only sidecar module wiring
