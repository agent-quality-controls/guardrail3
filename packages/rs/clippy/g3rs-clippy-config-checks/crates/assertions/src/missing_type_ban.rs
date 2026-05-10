crate::define_result_assertions!("g3rs-clippy/missing-type-ban");

/// Asserts the count of "missing type ban" findings equals `expected`.
///
/// # Panics
///
/// Panics if the count differs.
pub fn assert_missing_type_ban_count(
    results: &[guardrail3_check_types::G3CheckResult],
    expected: usize,
) {
    let actual = results
        .iter()
        .filter(|result| {
            result.id() == "g3rs-clippy/missing-type-ban" && result.title() == "missing type ban"
        })
        .count();
    assert_eq!(actual, expected, "{:#?}", findings(results));
}

/// Asserts at least one "missing type ban" finding's message contains `path`.
///
/// # Panics
///
/// Panics if no matching finding is present.
pub fn assert_contains_for_path(results: &[guardrail3_check_types::G3CheckResult], path: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-clippy/missing-type-ban"
                && result.title() == "missing type ban"
                && result.message().contains(path)
        }),
        "{:#?}",
        findings(results)
    );
}
