use crate::domain::modules::clippy::MAX_STRUCT_BOOLS;
use crate::domain::report::CheckResult;

use super::clippy_support::check_threshold_rule;
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-02";
const KEY: &str = "max-struct-bools";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    check_threshold_rule(input, results, ID, KEY, MAX_STRUCT_BOOLS);
}

#[cfg(test)]
#[path = "rs_clippy_02_max_struct_bools_tests.rs"]
mod tests;
