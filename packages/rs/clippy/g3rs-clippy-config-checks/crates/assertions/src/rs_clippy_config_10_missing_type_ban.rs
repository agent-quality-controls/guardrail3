crate::define_result_assertions!("RS-CLIPPY-CONFIG-10");

pub fn assert_missing_type_ban_count(
    results: &[guardrail3_check_types::G3CheckResult],
    expected: usize,
) {
    let actual = results
        .iter()
        .filter(|result| {
            result.id() == "RS-CLIPPY-CONFIG-10" && result.title() == "missing type ban"
        })
        .count();
    assert_eq!(actual, expected, "{:#?}", findings(results));
}

pub fn assert_contains_missing_type_ban(
    results: &[guardrail3_check_types::G3CheckResult],
    path: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-CLIPPY-CONFIG-10"
                && result.title() == "missing type ban"
                && result.message().contains(path)
        }),
        "{:#?}",
        findings(results)
    );
}
