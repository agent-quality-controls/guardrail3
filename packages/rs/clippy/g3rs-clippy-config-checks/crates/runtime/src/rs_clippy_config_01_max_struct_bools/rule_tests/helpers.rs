use guardrail3_check_types::G3CheckResult;

use super::super::check;
use test_support::{input_from_raw, input_with_raw_and_waivers, waiver};

pub(super) fn run_check(clippy_toml: &str) -> Vec<G3CheckResult> {
    let input = input_from_raw("clippy.toml", clippy_toml);
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

pub(super) fn run_check_with_waiver(clippy_toml: &str) -> Vec<G3CheckResult> {
    let input = input_with_raw_and_waivers(
        "clippy.toml",
        clippy_toml,
        g3rs_clippy_types::G3RsClippyRustPolicyState::Missing,
        false,
        Vec::new(),
        vec![waiver(
            "RS-CLIPPY-CONFIG-01",
            "clippy.toml",
            "key:max-struct-bools",
            "schema mirror",
        )],
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
