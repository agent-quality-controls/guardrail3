crate::define_result_assertions!("RS-CLIPPY-CONFIG-09");

pub fn assert_missing_method_ban_count(
    results: &[guardrail3_check_types::G3CheckResult],
    expected: usize,
) {
    let actual = results
        .iter()
        .filter(|result| {
            result.id() == "RS-CLIPPY-CONFIG-09" && result.title() == "missing method ban"
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
            result.id() == "RS-CLIPPY-CONFIG-09"
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
            result.id() == "RS-CLIPPY-CONFIG-09"
                && result.title() == "disallowed-methods section malformed"
                && result.message().contains(needle)
        }),
        "{:#?}",
        findings(results)
    );
}
