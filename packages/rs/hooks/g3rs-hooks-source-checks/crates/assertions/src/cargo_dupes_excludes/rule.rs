crate::define_rule_assertions!("g3rs-hooks/cargo-dupes-excludes");

pub fn assert_present(results: &[G3CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            title: Some("`.githooks/pre-commit` runs `cargo dupes --exclude-tests`"),
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
            title: Some("missing `--exclude-tests` on `cargo dupes` in `.githooks/pre-commit`"),
            message_contains: Some("test-only crates"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
