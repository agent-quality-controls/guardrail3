crate::define_result_assertions!("g3rs-clippy/macro-bans");

pub fn assert_macro_ban_present_count(
    results: &[guardrail3_check_types::G3CheckResult],
    expected: usize,
) {
    let actual = results
        .iter()
        .filter(|result| {
            result.id() == "g3rs-clippy/macro-bans" && result.title() == "macro ban present"
        })
        .count();
    assert_eq!(actual, expected, "{:#?}", findings(results));
}

pub fn assert_contains_missing_macro_ban(
    results: &[guardrail3_check_types::G3CheckResult],
    macro_name: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-clippy/macro-bans"
                && result.title() == "missing macro ban"
                && result.message().contains(macro_name)
        }),
        "{:#?}",
        findings(results)
    );
}

pub fn assert_contains_malformed_macro_section(
    results: &[guardrail3_check_types::G3CheckResult],
    needle: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-clippy/macro-bans"
                && result.title() == "disallowed-macros section malformed"
                && result.message().contains(needle)
        }),
        "{:#?}",
        findings(results)
    );
}
