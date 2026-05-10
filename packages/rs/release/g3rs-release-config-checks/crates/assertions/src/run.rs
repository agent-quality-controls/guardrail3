/// Assert that the result set is empty.
///
/// # Panics
/// Panics if `results` is not empty.
pub fn assert_no_findings(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(results.is_empty(), "{results:#?}");
}

/// Assert that exactly `expected_count` results carry the given id.
///
/// # Panics
/// Panics if the actual count of results matching `id` differs from `expected_count`.
pub fn assert_result_id_count(
    results: &[guardrail3_check_types::G3CheckResult],
    id: &str,
    expected_count: usize,
) {
    let actual = results.iter().filter(|result| result.id() == id).count();
    assert_eq!(actual, expected_count, "{results:#?}");
}

/// Assert that the result set contains a finding matching `id`, `severity`, and `title`.
///
/// # Panics
/// Panics if no result matches the supplied tuple.
pub fn assert_contains_result(
    results: &[guardrail3_check_types::G3CheckResult],
    id: &str,
    severity: guardrail3_check_types::G3Severity,
    title: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == id && result.severity() == severity && result.title() == title
        }),
        "{results:#?}"
    );
}
