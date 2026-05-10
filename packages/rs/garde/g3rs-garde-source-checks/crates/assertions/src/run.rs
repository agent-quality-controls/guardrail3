use guardrail3_check_types::G3CheckResult;

/// Implements this item.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub fn assert_no_results(results: &[G3CheckResult]) {
    assert!(results.is_empty(), "{results:#?}");
}

/// Implements this item.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub fn assert_contains_result(results: &[G3CheckResult], id: &str) {
    assert!(
        results.iter().any(|result| result.id() == id),
        "{results:#?}"
    );
}
