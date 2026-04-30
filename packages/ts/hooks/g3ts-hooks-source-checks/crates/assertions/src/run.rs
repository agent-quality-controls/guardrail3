use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_has_id(results: &[G3CheckResult], expected: &str, context: &str) {
    assert!(
        results.iter().any(|result| result.id() == expected),
        "expected result id `{expected}` for {context}, got {results:#?}"
    );
}

pub fn assert_missing_id(results: &[G3CheckResult], expected: &str, context: &str) {
    assert!(
        !results.iter().any(|result| result.id() == expected),
        "did not expect result id `{expected}` for {context}, got {results:#?}"
    );
}

pub fn assert_result_id_severity(
    results: &[G3CheckResult],
    result_id: &str,
    expected: G3Severity,
    context: &str,
) {
    let result = results
        .iter()
        .find(|result| result.id() == result_id)
        .expect("expected result id to be present before checking severity");
    assert_eq!(
        result.severity(),
        expected,
        "severity mismatch for {context}"
    );
}
