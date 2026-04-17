pub fn assert_no_findings(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(results.is_empty(), "{results:#?}");
}
