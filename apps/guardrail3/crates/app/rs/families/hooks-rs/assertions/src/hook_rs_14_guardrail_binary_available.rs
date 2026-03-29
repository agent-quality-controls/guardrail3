crate::define_rule_assertions!("HOOK-RS-14");

pub fn assert_present(results: &[guardrail3_domain_report::CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            title: Some("guardrail3 binary available"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

pub fn assert_missing(results: &[guardrail3_domain_report::CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            title: Some("guardrail3 binary missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
