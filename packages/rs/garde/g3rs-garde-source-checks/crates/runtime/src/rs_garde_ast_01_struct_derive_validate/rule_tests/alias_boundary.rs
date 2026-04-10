use g3rs_garde_source_checks_assertions::rs_garde_ast_01_struct_derive_validate as assertions;

#[test]
fn errors_for_aliased_deserialize_boundary_missing_validate() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/input.rs",
            "use serde::Deserialize as De;\n\nstruct Nested {\n    name: String,\n}\n\n#[derive(De)]\nstruct Input {\n    nested: Nested,\n}\n",
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
            ..Default::default()
        }],
    );
}
