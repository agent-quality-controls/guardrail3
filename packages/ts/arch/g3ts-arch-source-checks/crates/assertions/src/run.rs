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

pub fn assert_has_finding(
    results: &[guardrail3_check_types::G3CheckResult],
    id: &str,
    inventory: bool,
    title: &str,
    message: &str,
    file: Option<&str>,
    line: Option<usize>,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == id
                && result.inventory() == inventory
                && result.title() == title
                && result.message() == message
                && result.file() == file
                && result.line() == line
        }),
        "expected finding `{id}` ({title}), got {results:?}"
    );
}
