use guardrail3_domain_modules::clippy::MAX_STRUCT_BOOLS;
use guardrail3_domain_report::{CheckResult, Severity};

use crate::clippy_support::{IntegerSetting, integer_setting, value_kind};
use crate::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-02";
const KEY: &str = "max-struct-bools";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    match integer_setting(parsed, KEY) {
        IntegerSetting::Value(actual) if actual == MAX_STRUCT_BOOLS => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                format!("{KEY} correct"),
                format!("{KEY} = {MAX_STRUCT_BOOLS}"),
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
            format!("Expected {MAX_STRUCT_BOOLS}, got {actual}. Set `{KEY} = {MAX_STRUCT_BOOLS}` in clippy.toml."),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
        IntegerSetting::WrongType(value) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{KEY} wrong type"),
            format!(
                "Expected integer `{KEY} = {MAX_STRUCT_BOOLS}`, found {}.",
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
            format!("Add `{KEY} = {MAX_STRUCT_BOOLS}` to clippy.toml."),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
    }
}

