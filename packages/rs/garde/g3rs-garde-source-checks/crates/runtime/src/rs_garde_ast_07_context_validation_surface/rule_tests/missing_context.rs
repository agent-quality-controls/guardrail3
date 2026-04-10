use g3rs_garde_source_checks_assertions::rs_garde_ast_07_context_validation_surface as assertions;

#[test]
fn errors_when_ctx_usage_has_no_type_level_context() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/input.rs",
            "use garde::Validate;\nuse serde::Deserialize;\n\n#[derive(Deserialize, Validate)]\nstruct Input {\n    #[garde(length(chars, min = ctx.title_min, max = ctx.title_max))]\n    title: String,\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/input.rs"),
            title: Some("boundary `Input` uses ctx without garde(context)"),
            message_contains: Some("missing `#[garde(context(...))]`"),
            ..Default::default()
        }],
    );
}
