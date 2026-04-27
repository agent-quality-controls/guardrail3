crate::define_rule_assertions!("g3rs-hooks/clippy-denies-warnings");

pub fn assert_present(results: &[G3CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some("`.githooks/pre-commit` runs clippy in deny-warnings mode"),
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
            title: Some("missing deny-warnings `cargo clippy` command in `.githooks/pre-commit`"),
            message_contains: Some("hook fails on any clippy warning"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
