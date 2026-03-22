use crate::domain::modules::clippy::TOO_MANY_LINES_THRESHOLD;
use crate::domain::report::CheckResult;

use super::clippy_support::check_threshold_rule;
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-09";
const KEY: &str = "too-many-lines-threshold";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    check_threshold_rule(input, results, ID, KEY, TOO_MANY_LINES_THRESHOLD);
}

#[cfg(test)]
#[path = "rs_clippy_09_too_many_lines_threshold_tests.rs"]
mod tests;
