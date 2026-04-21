crate::define_rule_assertions!("RS-HOOKS-SOURCE-08");

pub fn assert_present(results: &[G3CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some(
                "`.githooks/pre-commit` uses `cargo dupes` for Rust dependency duplication",
            ),
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
            title: Some(
                "replace `jscpd` with `cargo dupes --exclude-tests` for Rust dependency duplication",
            ),
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
            title: Some("missing `cargo dupes --exclude-tests` command in `.githooks/pre-commit`"),
            message_contains: Some("duplicate Rust dependency versions"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
