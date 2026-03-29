crate::define_rule_assertions!("RS-DEPS-03");

pub fn assert_exactness_summary(results: &[guardrail3_domain_report::CheckResult]) {
    assert_eq!(results.len(), 4, "unexpected deps exactness results: {results:#?}");
    assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            inventory: Some(false),
            ..Default::default()
        }],
    );
    crate::common::assert_rule_results(
        results,
        "RS-DEPS-01",
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            inventory: Some(true),
            ..Default::default()
        }],
    );
    crate::common::assert_rule_results(
        results,
        "RS-DEPS-02",
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            inventory: Some(true),
            ..Default::default()
        }],
    );
    crate::common::assert_rule_results(
        results,
        "RS-DEPS-04",
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
