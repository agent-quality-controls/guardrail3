use g3rs_release_source_checks_types::G3RsReleaseSourceChecksInput;
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
mod tests {
    use super::check;
    use crate::test_support::{failure, source_input};

    #[test]
    fn aggregates_quality_and_input_failures() {
        let mut input = source_input(
            "# Demo\n\nThis crate has a heading and enough content to satisfy the README quality rule. \
This text keeps going so the README is comfortably above the stub threshold for release checks.",
        );
        input
            .input_failures
            .push(failure("crates/demo/README.md", "Failed to read README"));

        let results = check(&input);

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|result| result.id() == "RS-RELEASE-SOURCE-01"));
        assert!(results.iter().any(|result| result.id() == "RS-RELEASE-SOURCE-02"));
    }
}
