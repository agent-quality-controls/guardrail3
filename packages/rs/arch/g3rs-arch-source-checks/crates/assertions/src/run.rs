use guardrail3_check_types::G3CheckResult;

/// Internal.
///
/// # Panics
///
/// See body for assertions.
pub fn assert_has_finding_id(results: &[G3CheckResult], id: &str) {
    assert!(
        results.iter().any(|result| result.id() == id),
        "{results:#?}"
    );
}
