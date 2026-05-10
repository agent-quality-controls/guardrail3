crate::define_result_assertions!("g3rs-clippy/missing-method-ban");

/// Asserts the count of "missing method ban" findings equals `expected`.
///
/// # Panics
///
/// Panics if the count differs.
pub fn assert_missing_method_ban_count(
    results: &[guardrail3_check_types::G3CheckResult],
    expected: usize,
) {
    let actual = results
        .iter()
        .filter(|result| {
            result.id() == "g3rs-clippy/missing-method-ban"
                && result.title() == "missing method ban"
        })
        .count();
    assert_eq!(actual, expected, "{:#?}", findings(results));
}

/// Asserts at least one "missing method ban" finding's message contains `path`.
///
/// # Panics
///
/// Panics if no matching finding is present.
pub fn assert_contains_for_path(results: &[guardrail3_check_types::G3CheckResult], path: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-clippy/missing-method-ban"
                && result.title() == "missing method ban"
                && result.message().contains(path)
        }),
        "{:#?}",
        findings(results)
    );
}

/// Asserts at least one malformed-methods-section finding's message contains `needle`.
///
/// # Panics
///
/// Panics if no matching finding is present.
pub fn assert_contains_malformed_section(
    results: &[guardrail3_check_types::G3CheckResult],
    needle: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-clippy/missing-method-ban"
                && result.title() == "disallowed-methods section malformed"
                && result.message().contains(needle)
        }),
        "{:#?}",
        findings(results)
    );
}
