crate::define_result_assertions!("g3rs-clippy/missing-type-ban");

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

pub fn assert_contains_missing_type_ban(
    results: &[guardrail3_check_types::G3CheckResult],
    path: &str,
) {
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
