use g3rs_clippy_types::G3RsClippyFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run extracted clippy file-tree checks against the input snapshot.
#[must_use]
pub fn check(input: &G3RsClippyFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::coverage_exists::check(input, &mut results);
    crate::same_root_conflict::check(input, &mut results);

    results
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
