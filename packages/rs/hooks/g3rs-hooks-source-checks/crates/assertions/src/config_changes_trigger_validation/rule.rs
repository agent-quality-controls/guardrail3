crate::define_rule_assertions!("g3rs-hooks/config-changes-trigger-validation");

pub fn assert_present(results: &[G3CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some(
                "`.githooks/pre-commit` triggers Rust validation on guardrail config changes",
            ),
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
            title: Some(
                "incomplete Rust guardrail config trigger coverage in `.githooks/pre-commit`",
            ),
            message_contains: Some("config-only policy change"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
