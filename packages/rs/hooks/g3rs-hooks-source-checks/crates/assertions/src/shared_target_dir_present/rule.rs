crate::define_rule_assertions!("g3rs-hooks/shared-target-dir-present");

pub fn assert_present(results: &[G3CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some("`.githooks/pre-commit` sets a shared `CARGO_TARGET_DIR`"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

pub fn assert_missing(results: &[G3CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some("missing shared `CARGO_TARGET_DIR` setup in `.githooks/pre-commit`"),
            message_contains: Some("export CARGO_TARGET_DIR"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
