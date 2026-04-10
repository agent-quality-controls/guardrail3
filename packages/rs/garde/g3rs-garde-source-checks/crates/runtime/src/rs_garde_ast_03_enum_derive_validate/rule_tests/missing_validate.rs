use g3rs_garde_source_checks_assertions::rs_garde_ast_03_enum_derive_validate as assertions;

#[test]
fn errors_when_enum_boundary_is_missing_validate() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/input.rs",
            "use serde::Deserialize;\n\n#[derive(Deserialize)]\nenum Input {\n    Variant(String),\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/input.rs"),
            title: Some("enum `Input` missing Validate derive"),
            message_contains: Some("non-primitive payload fields"),
            ..Default::default()
        }],
    );
}
