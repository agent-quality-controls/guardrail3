use guardrail3_check_types::G3CheckResult;

/// Asserts that at least one finding in `results` has rule `id`.
///
/// # Panics
///
/// Panics when no finding with rule `id` exists.
pub fn assert_has_finding_id(results: &[G3CheckResult], id: &str) {
    assert!(
        results.iter().any(|result| result.id() == id),
        "{results:#?}"
    );
}

/// Asserts that no finding in `results` has rule `id`.
///
/// # Panics
///
/// Panics when at least one finding with rule `id` exists.
pub fn assert_missing_finding_id(results: &[G3CheckResult], id: &str) {
    assert!(
        !results.iter().any(|result| result.id() == id),
        "{results:#?}"
    );
}
