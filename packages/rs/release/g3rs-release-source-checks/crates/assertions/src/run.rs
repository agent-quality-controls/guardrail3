/// Asserts that `results` ids match `expected` exactly, in order.
///
/// # Panics
///
/// Panics when the rule ids in `results` do not match `expected`.
pub fn assert_result_ids(results: &[guardrail3_check_types::G3CheckResult], expected: &[&str]) {
    let ids = results
        .iter()
        .map(guardrail3_check_types::G3CheckResult::id)
        .collect::<Vec<_>>();
    assert_eq!(ids, expected, "rule id ordering mismatch");
}
