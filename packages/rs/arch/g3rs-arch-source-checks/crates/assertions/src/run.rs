use guardrail3_check_types::G3CheckResult;

pub fn assert_has_finding_id(results: &[G3CheckResult], id: &str) {
    assert!(results.iter().any(|result| result.id() == id), "{results:#?}");
}
