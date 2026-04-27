crate::define_result_assertions!("g3rs-clippy/missing-method-ban");

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

pub fn assert_contains_missing_method_ban(
    results: &[guardrail3_check_types::G3CheckResult],
    path: &str,
) {
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
