use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_has_result(
    results: &[G3CheckResult],
    id: &str,
    severity: G3Severity,
    file: Option<&str>,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == id
                && result.severity() == severity
                && file.is_none_or(|expected| result.file() == Some(expected))
        }),
        "{results:#?}"
    );
}

pub fn assert_missing_result(results: &[G3CheckResult], id: &str) {
    assert!(
        !results.iter().any(|result| result.id() == id),
        "{results:#?}"
    );
}

pub fn assert_missing_result_with_severity(
    results: &[G3CheckResult],
    id: &str,
    severity: G3Severity,
) {
    assert!(
        !results
            .iter()
            .any(|result| result.id() == id && result.severity() == severity),
        "{results:#?}"
    );
}
