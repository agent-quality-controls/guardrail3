pub fn assert_no_findings(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(results.is_empty(), "{results:#?}");
}

pub fn assert_result_id_count(
    results: &[guardrail3_check_types::G3CheckResult],
    id: &str,
    expected_count: usize,
) {
    let actual = results.iter().filter(|result| result.id() == id).count();
    assert_eq!(actual, expected_count, "{results:#?}");
}

pub fn assert_contains_result(
    results: &[guardrail3_check_types::G3CheckResult],
    id: &str,
    severity: guardrail3_check_types::G3Severity,
    title: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == id && result.severity() == severity && result.title() == title
        }),
        "{results:#?}"
    );
}
