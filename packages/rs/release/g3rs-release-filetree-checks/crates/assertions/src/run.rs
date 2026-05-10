/// Assert no runtime findings were produced.
///
/// # Panics
///
/// Panics when `results` is not empty.
pub fn assert_no_findings(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(results.is_empty(), "expected no findings, got {results:#?}");
}

/// Assert the produced finding ids match `expected` in order.
///
/// # Panics
///
/// Panics when the finding id sequence does not match `expected`.
pub fn assert_result_ids(results: &[guardrail3_check_types::G3CheckResult], expected: &[&str]) {
    let ids = results
        .iter()
        .map(guardrail3_check_types::G3CheckResult::id)
        .collect::<Vec<_>>();
    assert_eq!(ids, expected, "finding id sequence mismatch");
}
