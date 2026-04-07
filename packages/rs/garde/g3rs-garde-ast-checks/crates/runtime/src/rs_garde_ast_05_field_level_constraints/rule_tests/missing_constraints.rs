use g3rs_garde_ast_checks_assertions::rs_garde_ast_05_field_level_constraints as assertions;

#[test]
fn errors_when_validated_boundary_field_has_no_real_garde_rule() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/input.rs",
            "use garde::Validate;\nuse serde::Deserialize;\n\n#[derive(Deserialize, Validate)]\nstruct Input {\n    name: String,\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/input.rs"),
            title: Some("boundary field `name` missing garde validator"),
            message_contains: Some("no meaningful garde validator"),
            ..Default::default()
        }],
    );
}
