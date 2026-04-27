use g3rs_release_types::G3RsReleaseInputFailure;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-release/filetree-input-failures";

pub(crate) fn check(failure: &G3RsReleaseInputFailure, results: &mut Vec<G3CheckResult>) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "failed to read release filetree input".to_owned(),
        failure.message.clone(),
        Some(failure.rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "input_failures_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod input_failures_tests;
