use guardrail3_domain_modules::clippy::MAX_FN_PARAMS_BOOLS;
use guardrail3_domain_report::{CheckResult, Severity};

use crate::clippy_support::{IntegerSetting, integer_setting, value_kind};
use crate::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-03";
const KEY: &str = "max-fn-params-bools";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    match integer_setting(parsed, KEY) {
        IntegerSetting::Value(actual) if actual == MAX_FN_PARAMS_BOOLS => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                format!("{KEY} correct"),
                format!("{KEY} = {MAX_FN_PARAMS_BOOLS}"),
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
            format!("Expected {MAX_FN_PARAMS_BOOLS}, got {actual}. Set `{KEY} = {MAX_FN_PARAMS_BOOLS}` in clippy.toml."),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
        IntegerSetting::WrongType(value) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("{KEY} wrong type"),
            format!(
                "Expected integer `{KEY} = {MAX_FN_PARAMS_BOOLS}`, found {}.",
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
            format!("Add `{KEY} = {MAX_FN_PARAMS_BOOLS}` to clippy.toml."),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
    }
}

