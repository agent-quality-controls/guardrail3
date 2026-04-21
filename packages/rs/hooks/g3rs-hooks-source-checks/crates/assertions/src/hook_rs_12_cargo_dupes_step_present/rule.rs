crate::define_rule_assertions!("RS-HOOKS-SOURCE-13");

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
