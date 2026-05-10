crate::define_rule_assertions!("g3rs-hooks/routing-discovers-marker-pair");

pub fn assert_error_finding(results: &[G3CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
