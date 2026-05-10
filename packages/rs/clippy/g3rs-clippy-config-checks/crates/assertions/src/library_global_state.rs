crate::define_result_assertions!("g3rs-clippy/library-global-state");

/// Asserts the count of "library clippy.toml missing global-state type ban" findings equals `expected`.
///
/// # Panics
///
/// Panics if the count differs.
pub fn assert_missing_global_state_ban_count(
    results: &[guardrail3_check_types::G3CheckResult],
    expected: usize,
) {
    let actual = results
        .iter()
        .filter(|result| {
            result.id() == "g3rs-clippy/library-global-state"
                && result.title() == "library clippy.toml missing global-state type ban"
        })
        .count();
    assert_eq!(actual, expected, "{:#?}", findings(results));
}

/// Asserts at least one missing-global-state finding's message contains `path`.
///
/// # Panics
///
/// Panics if no matching finding is present.
pub fn assert_contains_missing_global_state_ban(
    results: &[guardrail3_check_types::G3CheckResult],
    path: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-clippy/library-global-state"
                && result.title() == "library clippy.toml missing global-state type ban"
                && result.message().contains(path)
        }),
        "{:#?}",
        findings(results)
    );
}
