use g3rs_deps_types::G3RsDepsFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3RsDepsFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::cargo_lock_present::check(input, &mut results);
    crate::gitignore_not_ignoring_cargo_lock::check(input, &mut results);

    results
}
