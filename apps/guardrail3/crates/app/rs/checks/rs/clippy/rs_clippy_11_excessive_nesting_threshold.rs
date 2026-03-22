use crate::domain::modules::clippy::EXCESSIVE_NESTING_THRESHOLD;
use crate::domain::report::CheckResult;

use super::clippy_support::check_threshold_rule;
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-11";
const KEY: &str = "excessive-nesting-threshold";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    check_threshold_rule(input, results, ID, KEY, EXCESSIVE_NESTING_THRESHOLD);
}

#[cfg(test)]
#[path = "rs_clippy_11_excessive_nesting_threshold_tests.rs"]
mod tests;
