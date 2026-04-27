use g3rs_deps_types::G3RsDepsFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsDepsFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::cargo_lock_present::check(input, &mut results);
    crate::gitignore_not_ignoring_cargo_lock::check(input, &mut results);

    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
