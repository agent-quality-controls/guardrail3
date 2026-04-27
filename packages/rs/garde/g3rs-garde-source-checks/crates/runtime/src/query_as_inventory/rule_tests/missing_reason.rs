use g3rs_garde_source_checks_assertions::query_as_inventory::rule as assertions;

#[test]
fn errors_when_query_as_has_no_escape_hatch_reason() {
    let fixture = super::helpers::fixture(vec![super::helpers::macro_use(
        "src/db.rs",
        2,
        "sqlx::query_as",
        true,
        None,
    )]);

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
