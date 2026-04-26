pub fn assert_has_error(results: &[guardrail3_check_types::G3CheckResult], id: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.id() == id && !result.inventory()),
        "expected error `{id}`, got {results:?}"
    );
}

pub fn assert_has_inventory(results: &[guardrail3_check_types::G3CheckResult], id: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.id() == id && result.inventory()),
        "expected inventory `{id}`, got {results:?}"
    );
}

pub fn assert_missing(results: &[guardrail3_check_types::G3CheckResult], id: &str) {
    assert!(
        results.iter().all(|result| result.id() != id),
        "expected no finding `{id}`, got {results:?}"
    );
}

pub fn assert_error_files(
    results: &[guardrail3_check_types::G3CheckResult],
    id: &str,
    expected: &[&str],
) {
    let actual = results
        .iter()
        .filter(|result| result.id() == id && !result.inventory())
        .map(|result| result.file().unwrap_or("<missing>").to_owned())
        .collect::<Vec<_>>();
    let expected = expected
        .iter()
        .map(|item| (*item).to_owned())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "error file list mismatch for `{id}`");
}
