pub fn assert_result_id_count(
    results: &[guardrail3_check_types::G3CheckResult],
    id: &str,
    expected_count: usize,
) {
    let actual = results.iter().filter(|result| result.id() == id).count();
    assert_eq!(actual, expected_count, "{results:#?}");
}
