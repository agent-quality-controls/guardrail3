use guardrail3_check_types::G3CheckResult;

/// Asserts that `results` is empty.
///
/// # Panics
///
/// Panics when `results` is non-empty.
pub fn assert_no_results(results: &[G3CheckResult]) {
    assert!(results.is_empty(), "{results:#?}");
}

/// Asserts that the rule with `id` emitted at least one finding pointing at `file`.
///
/// # Panics
///
/// Panics when no such finding exists in `results`.
pub fn assert_rule_present(results: &[G3CheckResult], id: &str, file: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.id() == id && result.file() == Some(file)),
        "{results:#?}"
    );
}

/// Asserts that no finding in `results` matches the `(id, title)` pair.
///
/// # Panics
///
/// Panics when at least one matching finding exists.
pub fn assert_rule_absent(results: &[G3CheckResult], id: &str, title: &str) {
    assert!(
        results
            .iter()
            .all(|result| !(result.id() == id && result.title() == title)),
        "{results:#?}"
    );
}

/// Asserts that no finding in `results` has rule `id`.
///
/// # Panics
///
/// Panics when at least one finding with rule `id` exists.
pub fn assert_rule_id_absent(results: &[G3CheckResult], id: &str) {
    assert!(
        results.iter().all(|result| result.id() != id),
        "{results:#?}"
    );
}
