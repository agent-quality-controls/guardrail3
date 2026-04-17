#[must_use]
pub fn count(results: &[guardrail3_check_types::G3CheckResult], id: &str) -> usize {
    results.iter().filter(|result| result.id() == id).count()
}

pub fn assert_present(
    results: &[guardrail3_check_types::G3CheckResult],
    id: &str,
    title: &str,
    file: Option<&str>,
    inventory: bool,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == id
                && result.title() == title
                && result.file() == file
                && result.inventory() == inventory
        }),
        "{results:#?}"
    );
}

pub fn assert_no_results(results: &[guardrail3_check_types::G3CheckResult], id: &str) {
    assert!(
        results.iter().all(|result| result.id() != id),
        "{results:#?}"
    );
}
