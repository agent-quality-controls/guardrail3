use g3rs_garde_source_checks_assertions::rs_garde_ast_04_query_as_inventory::rule as assertions;

#[test]
fn inventories_documented_query_as_usages() {
    let reason = "Temporary SQLx row mapping until validated DTO extraction lands.";
    let fixture = super::helpers::fixture(vec![
        super::helpers::macro_use("src/db.rs", 4, "sqlx::query_as", true, Some(reason)),
        super::helpers::macro_use("src/db.rs", 5, "qa", true, Some(reason)),
    ]);

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                severity: Some(assertions::G3Severity::Warn),
                file: Some("src/db.rs"),
                title: Some("sqlx query_as requires validation review"),
                message_contains: Some(
                    "`sqlx::query_as` bypasses derive-based garde boundary checks",
                ),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::G3Severity::Warn),
                file: Some("src/db.rs"),
                title: Some("sqlx query_as requires validation review"),
                message_contains: Some("`qa` bypasses derive-based garde boundary checks"),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::G3Severity::Warn),
                title: Some("sqlx query_as count"),
                message: Some("`src/db.rs` has 2 sqlx query_as escape hatches."),
                ..Default::default()
            },
        ],
    );
}
