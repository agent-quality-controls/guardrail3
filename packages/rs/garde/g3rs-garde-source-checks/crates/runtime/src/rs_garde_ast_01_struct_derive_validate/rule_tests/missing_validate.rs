use g3rs_garde_source_checks_assertions::rs_garde_ast_01_struct_derive_validate::rule as assertions;

#[test]
fn errors_when_struct_boundary_is_missing_validate() {
    let fixture = super::helpers::fixture(
        &[(
            "src/input.rs",
            "use serde::Deserialize;\n\n#[derive(Deserialize)]\nstruct Input {\n    name: String,\n}\n",
        )],
        super::helpers::default_guardrail_toml(),
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
