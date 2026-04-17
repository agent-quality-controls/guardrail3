use g3rs_deny_types::G3RsDenyFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsDenyFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::rs_deny_filetree_01_coverage::check(input, &mut results);
    crate::rs_deny_filetree_03_shadowing::check(input, &mut results);

    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
// reason: file module tests live in the owned run_tests sidecar directory.
mod run_tests;
