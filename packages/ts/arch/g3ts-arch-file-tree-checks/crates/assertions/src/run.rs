/// Assert a non-inventory finding with the given id is present.
///
/// # Panics
///
/// Panics if no matching error finding is present in `results`.
pub fn assert_has_error(results: &[guardrail3_check_types::G3CheckResult], id: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.id() == id && !result.inventory()),
        "expected error `{id}`, got {results:?}"
    );
}

/// Assert an inventory finding with the given id is present.
///
/// # Panics
///
/// Panics if no matching inventory finding is present in `results`.
pub fn assert_has_inventory(results: &[guardrail3_check_types::G3CheckResult], id: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.id() == id && result.inventory()),
        "expected inventory `{id}`, got {results:?}"
    );
}

/// Assert no finding with the given id is present.
///
/// # Panics
///
/// Panics if any finding in `results` has the given id.
pub fn assert_missing(results: &[guardrail3_check_types::G3CheckResult], id: &str) {
    assert!(
        results.iter().all(|result| result.id() != id),
        "expected no finding `{id}`, got {results:?}"
    );
}
