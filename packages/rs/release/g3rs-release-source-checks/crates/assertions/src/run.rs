pub fn assert_result_ids(results: &[guardrail3_check_types::G3CheckResult], expected: &[&str]) {
    let ids = results
        .iter()
        .map(guardrail3_check_types::G3CheckResult::id)
        .collect::<Vec<_>>();
    assert_eq!(ids, expected);
}
