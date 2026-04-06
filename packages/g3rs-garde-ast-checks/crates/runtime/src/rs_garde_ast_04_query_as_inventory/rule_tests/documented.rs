use g3rs_garde_ast_checks_assertions::rs_garde_ast_04_query_as_inventory as assertions;

#[test]
fn inventories_documented_query_as_usages() {
    let guardrail = "[profile]\nname = \"service\"\n\n[[escape_hatches]]\nfamily = \"garde\"\nfile = \"src/db.rs\"\nkind = \"sqlx_query_as\"\nselector = \"sqlx::query_as@L4\"\nreason = \"Temporary SQLx row mapping until validated DTO extraction lands.\"\n\n[[escape_hatches]]\nfamily = \"garde\"\nfile = \"src/db.rs\"\nkind = \"sqlx_query_as\"\nselector = \"qa@L5\"\nreason = \"Temporary SQLx row mapping until validated DTO extraction lands.\"\n";
    let fixture = crate::test_support::fixture(
        &[(
            "src/db.rs",
            "use sqlx::query_as as qa;\n\nfn load() {\n    let _row = sqlx::query_as!(User, \"select 1\");\n    let _row2 = qa!(User, \"select 2\");\n}\n",
        )],
        guardrail,
    );

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                severity: Some(assertions::G3Severity::Warn),
                file: Some("src/db.rs"),
                title: Some("sqlx query_as requires validation review"),
                message_contains: Some("`sqlx::query_as` bypasses derive-based garde boundary checks"),
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
