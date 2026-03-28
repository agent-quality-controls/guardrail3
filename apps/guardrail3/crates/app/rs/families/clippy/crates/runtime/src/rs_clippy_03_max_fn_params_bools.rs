use guardrail3_domain_modules::clippy::MAX_FN_PARAMS_BOOLS;
#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::threshold_value;
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-03";
const KEY: &str = "max-fn-params-bools";

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
        Some(actual) if actual == MAX_FN_PARAMS_BOOLS => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: format!("{KEY} correct"),
                message: format!("{KEY} = {MAX_FN_PARAMS_BOOLS}"),
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
            message: format!("Expected {MAX_FN_PARAMS_BOOLS}, got {actual}."),
            file: Some(input.config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("{KEY} missing"),
            message: format!("Expected {KEY} = {MAX_FN_PARAMS_BOOLS}."),
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
#[path = "rs_clippy_03_max_fn_params_bools_tests/mod.rs"]
mod rs_clippy_03_max_fn_params_bools_tests;
