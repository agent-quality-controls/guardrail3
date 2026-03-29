crate::define_rule_assertions!("HOOK-RS-08");

pub fn assert_present(results: &[guardrail3_domain_report::CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            title: Some("Rust guardrail validate step present"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

pub fn assert_missing(results: &[guardrail3_domain_report::CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some("Rust guardrail validate step missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
