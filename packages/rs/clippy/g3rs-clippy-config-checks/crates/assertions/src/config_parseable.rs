crate::define_result_assertions!("g3rs-clippy/config-parseable");

/// Asserts that at least one finding's message contains `needle` for the parse-error title.
///
/// # Panics
///
/// Panics if no matching finding is present.
pub fn assert_parse_error_contains(
    results: &[guardrail3_check_types::G3CheckResult],
    needle: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-clippy/config-parseable"
                && result.title() == "clippy.toml parse error"
                && result.message().contains(needle)
        }),
        "{:#?}",
        findings(results)
    );
}
