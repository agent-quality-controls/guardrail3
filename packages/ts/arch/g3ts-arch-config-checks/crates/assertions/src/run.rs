/// Assert that `results` contains a non-inventory entry with rule `id`.
///
/// # Panics
///
/// Panics when no non-inventory result with `id` is present.
pub fn assert_has_error(results: &[guardrail3_check_types::G3CheckResult], id: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.id() == id && !result.inventory()),
        "expected error `{id}`, got {results:?}"
    );
}
