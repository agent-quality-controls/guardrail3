use g3rs_release_types::G3RsReleaseSourceChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3RsReleaseSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for failure in &input.input_failures {
        crate::input_failures::check(failure, &mut results);
    }

    for readme in &input.readmes {
        crate::readme_quality::check(readme, &mut results);
    }

    results
}
