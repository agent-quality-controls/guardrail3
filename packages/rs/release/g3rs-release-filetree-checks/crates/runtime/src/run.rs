use g3rs_release_types::G3RsReleaseFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run all release file-tree checks against `input` and return collected findings.
#[must_use]
pub fn check(input: &G3RsReleaseFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for failure in &input.input_failures {
        crate::input_failures::check(failure, &mut results);
    }

    if let Some(repo) = &input.repo {
        crate::license_file::check(repo, &mut results);
        crate::release_plz_exists::check(repo, &mut results);
        crate::cliff_exists::check(repo, &mut results);
    }

    for readme in &input.readmes {
        crate::readme_exists::check(readme, &mut results);
    }

    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
