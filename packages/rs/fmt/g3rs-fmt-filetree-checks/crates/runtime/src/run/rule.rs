use g3rs_fmt_types::G3RsFmtFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsFmtFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::exists::check(input, &mut results);
    crate::per_crate_override::check(input, &mut results);
    crate::dual_file_conflict::check(input, &mut results);

    results
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
