crate::define_rule_assertions!("g3rs-hooks/cargo-dupes-step-present");

pub fn assert_present(results: &[G3CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some("`.githooks/pre-commit` runs `cargo dupes`"),
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
            title: Some("missing executable `cargo dupes` command in `.githooks/pre-commit`"),
            message_contains: Some("duplicate Rust dependency versions"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
