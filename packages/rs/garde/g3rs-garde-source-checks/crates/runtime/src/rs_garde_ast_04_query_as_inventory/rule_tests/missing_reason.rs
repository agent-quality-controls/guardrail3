use g3rs_garde_source_checks_assertions::rs_garde_ast_04_query_as_inventory::rule as assertions;

#[test]
fn errors_when_query_as_has_no_escape_hatch_reason() {
    let fixture = super::helpers::fixture(
        &[(
            "src/db.rs",
            "fn load() {\n    let _row = sqlx::query_as!(User, \"select 1\");\n}\n",
        )],
        super::helpers::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                severity: Some(assertions::G3Severity::Error),
                file: Some("src/db.rs"),
                title: Some("sqlx query_as missing reason"),
                message_contains: Some("without a matching waiver reason"),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::G3Severity::Warn),
                title: Some("sqlx query_as count"),
                message: Some("`src/db.rs` has 1 sqlx query_as escape hatches."),
                ..Default::default()
            },
        ],
    );
}
