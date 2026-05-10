crate::define_rule_assertions!("g3rs-hooks/skips-when-no-owning-unit");

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

pub fn assert_inventory_only(results: &[G3CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
