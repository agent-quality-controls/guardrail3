use g3rs_release_types::G3RsReleaseInputFailure;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3rs-release/source-input-failures";

/// Surfaces a release source input failure as a finding.
pub(crate) fn check(failure: &G3RsReleaseInputFailure, results: &mut Vec<G3CheckResult>) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "failed to read release source input".to_owned(),
        failure.message.clone(),
        Some(failure.rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "input_failures_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod input_failures_tests;
