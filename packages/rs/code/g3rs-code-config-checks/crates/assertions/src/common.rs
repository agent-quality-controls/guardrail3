use guardrail3_check_types::G3CheckResult;

pub fn require_single_result(results: &[G3CheckResult]) -> &G3CheckResult {
    assert_eq!(results.len(), 1, "unexpected results: {results:#?}");
    &results[0]
}
