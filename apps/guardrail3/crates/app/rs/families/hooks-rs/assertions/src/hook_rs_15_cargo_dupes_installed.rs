crate::define_rule_assertions!("HOOK-RS-15");

pub fn assert_present(results: &[guardrail3_domain_report::CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            title: Some("cargo-dupes installed"),
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
            title: Some("cargo-dupes missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
