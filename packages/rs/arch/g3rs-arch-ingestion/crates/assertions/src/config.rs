use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Asserts that `results` contains at least one finding matching `id`, `severity`, and `file`.
///
/// # Panics
///
/// Panics when no matching finding is present in `results`.
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

/// Asserts that no finding in `results` has the given `id`.
///
/// # Panics
///
/// Panics when at least one finding with `id` is present in `results`.
pub fn assert_missing_result(results: &[G3CheckResult], id: &str) {
    assert!(
        !results.iter().any(|result| result.id() == id),
        "{results:#?}"
    );
}
