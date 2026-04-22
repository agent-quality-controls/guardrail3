use g3rs_garde_source_checks_assertions::rs_garde_ast_07_context_validation_surface::rule as assertions;

#[test]
fn errors_when_ctx_usage_has_no_type_level_context() {
    let fixture = super::helpers::fixture(vec![super::helpers::field(
        "src/input.rs",
        6,
        "Input",
        "title",
    )]);

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
