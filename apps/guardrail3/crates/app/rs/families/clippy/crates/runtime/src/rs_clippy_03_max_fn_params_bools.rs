use guardrail3_domain_modules::clippy::MAX_FN_PARAMS_BOOLS;
#[cfg(test)]
use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{IntegerSetting, integer_setting, value_kind};
use super::inputs::ConfigClippyInput;

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
            format!("Expected {MAX_FN_PARAMS_BOOLS}, got {actual}."),
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
            format!("Expected {KEY} = {MAX_FN_PARAMS_BOOLS}."),
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
#[path = "rs_clippy_03_max_fn_params_bools_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_clippy_03_max_fn_params_bools_tests;
