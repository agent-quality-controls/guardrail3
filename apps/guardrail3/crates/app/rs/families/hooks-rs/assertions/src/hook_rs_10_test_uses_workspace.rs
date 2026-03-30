crate::define_rule_assertions!("HOOK-RS-10");

pub fn assert_present(results: &[guardrail3_domain_report::CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            title: Some("cargo test uses workspace scope"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

pub fn assert_missing(results: &[guardrail3_domain_report::CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            title: Some("cargo test missing --workspace"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
