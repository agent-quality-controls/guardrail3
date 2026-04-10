crate::define_rule_assertions!("HOOK-RS-07");

pub fn assert_present(results: &[G3CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some("cargo-dupes selected for Rust duplication checks"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

pub fn assert_wrong_tool(results: &[G3CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some("wrong Rust duplication tool"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

pub fn assert_missing(results: &[G3CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some("Rust duplication tool missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
