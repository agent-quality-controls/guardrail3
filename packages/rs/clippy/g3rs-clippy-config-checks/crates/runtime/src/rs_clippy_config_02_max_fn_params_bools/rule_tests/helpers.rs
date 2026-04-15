use guardrail3_check_types::G3CheckResult;

use super::super::rule::check;
use test_support::input_from_raw;

pub(super) fn run_check(clippy_toml: &str) -> Vec<G3CheckResult> {
    let input = input_from_raw("clippy.toml", clippy_toml);
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
