use g3rs_deps_filetree_checks_types::G3RsDepsFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsDepsFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::rs_deps_filetree_09_cargo_lock_present::check(input, &mut results);
    crate::rs_deps_filetree_10_gitignore_not_ignoring_cargo_lock::check(input, &mut results);

    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod tests;
