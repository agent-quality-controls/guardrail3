use g3rs_release_types::G3RsReleaseSourceChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsReleaseSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for failure in &input.input_failures {
        crate::rs_release_source_02_input_failures::check(failure, &mut results);
    }

    for readme in &input.readmes {
        crate::rs_release_source_01_readme_quality::check(readme, &mut results);
    }

    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
