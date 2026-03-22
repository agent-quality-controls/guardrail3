use crate::domain::modules::clippy::TOO_MANY_ARGUMENTS_THRESHOLD;
use crate::domain::report::CheckResult;

use super::clippy_support::check_threshold_rule;
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-10";
const KEY: &str = "too-many-arguments-threshold";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    check_threshold_rule(input, results, ID, KEY, TOO_MANY_ARGUMENTS_THRESHOLD);
}

#[cfg(test)]
#[path = "rs_clippy_10_too_many_arguments_threshold_tests.rs"]
mod tests;
