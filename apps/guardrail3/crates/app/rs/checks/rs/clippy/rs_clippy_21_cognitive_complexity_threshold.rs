use crate::domain::modules::clippy::COGNITIVE_COMPLEXITY_THRESHOLD;
use crate::domain::report::CheckResult;

use super::clippy_support::check_threshold_rule;
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-21";
const KEY: &str = "cognitive-complexity-threshold";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    check_threshold_rule(input, results, ID, KEY, COGNITIVE_COMPLEXITY_THRESHOLD);
}

#[cfg(test)]
#[path = "rs_clippy_21_cognitive_complexity_threshold_tests.rs"]
mod tests;
