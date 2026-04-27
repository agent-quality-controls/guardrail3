crate::define_rule_assertions!("g3rs-hooks/guardrail-validate-staged-present");

pub fn assert_present(results: &[G3CheckResult]) {
    self::assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            title: Some("`.githooks/pre-commit` runs `g3rs validate --path ...`"),
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
            title: Some("missing `g3rs validate --path ...` command in `.githooks/pre-commit`"),
            message_contains: Some("Cargo tools do not cover"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
