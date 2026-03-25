use guardrail3_domain_report::{CheckResult, Severity};

/// Filter results to errors matching a specific check ID.
pub fn errors_by_id<'a>(results: &'a [CheckResult], id: &str) -> Vec<&'a CheckResult> {
    results
        .iter()
        .filter(|r| r.id == id && r.severity == Severity::Error)
        .collect()
}

/// Assert exactly 1 error with title containing the given fragment.
pub fn assert_single_error(errors: &[&CheckResult], expected_title_fragment: &str) {
    assert_eq!(
        errors.len(),
        1,
        "expected exactly 1 error, got {}: {errors:#?}",
        errors.len()
    );
    assert!(
        errors[0].title.contains(expected_title_fragment),
        "expected title containing '{expected_title_fragment}', got: '{}'",
        errors[0].title
    );
}

/// Assert every error has the `file` field set.
pub fn assert_file_field(errors: &[&CheckResult]) {
    for err in errors {
        assert!(
            err.file.is_some(),
            "expected file field to be set, got None for: {err:#?}"
        );
    }
}
