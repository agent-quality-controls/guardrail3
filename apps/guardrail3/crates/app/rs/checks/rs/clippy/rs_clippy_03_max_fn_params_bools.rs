use crate::domain::modules::clippy::MAX_FN_PARAMS_BOOLS;
use crate::domain::report::CheckResult;

use super::clippy_support::check_threshold_rule;
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-03";
const KEY: &str = "max-fn-params-bools";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    check_threshold_rule(input, results, ID, KEY, MAX_FN_PARAMS_BOOLS);
}

#[cfg(test)]
#[path = "rs_clippy_03_max_fn_params_bools_tests.rs"]
mod tests;
