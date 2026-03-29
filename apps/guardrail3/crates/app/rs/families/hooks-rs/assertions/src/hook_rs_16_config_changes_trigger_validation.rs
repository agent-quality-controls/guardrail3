crate::define_rule_assertions!("HOOK-RS-16");

pub fn assert_present(results: &[guardrail3_domain_report::CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some("Rust config changes trigger hook validation"),
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
            title: Some("Rust config-change trigger coverage incomplete"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
