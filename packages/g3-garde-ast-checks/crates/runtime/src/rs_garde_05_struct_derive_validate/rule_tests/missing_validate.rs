use g3_garde_ast_checks_assertions::rs_garde_05_struct_derive_validate as assertions;

#[test]
fn errors_when_struct_boundary_is_missing_validate() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/input.rs",
            "use serde::Deserialize;\n\n#[derive(Deserialize)]\nstruct Input {\n    name: String,\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/input.rs"),
            title: Some("struct `Input` missing Validate derive"),
            message_contains: Some("does not derive garde's `Validate`"),
            ..Default::default()
        }],
    );
}
