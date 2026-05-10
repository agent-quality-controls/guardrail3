/// Asserts that `results` contains a non-inventory finding for `id`.
///
/// # Panics
///
/// Panics when no non-inventory finding for `id` is present.
pub fn assert_has_error(results: &[guardrail3_check_types::G3CheckResult], id: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.id() == id && !result.inventory()),
        "expected error `{id}`, got {results:?}"
    );
}

/// Asserts that `results` contains an inventory finding for `id`.
///
/// # Panics
///
/// Panics when no inventory finding for `id` is present.
pub fn assert_has_inventory(results: &[guardrail3_check_types::G3CheckResult], id: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.id() == id && result.inventory()),
        "expected inventory `{id}`, got {results:?}"
    );
}
