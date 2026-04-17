use guardrail3_check_types::G3CheckResult;

pub fn assert_no_results(results: &[G3CheckResult]) {
    assert!(results.is_empty(), "{results:#?}");
}
