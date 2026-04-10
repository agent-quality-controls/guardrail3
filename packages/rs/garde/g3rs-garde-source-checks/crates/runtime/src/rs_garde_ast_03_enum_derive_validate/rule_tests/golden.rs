use g3rs_garde_source_checks_assertions::rs_garde_ast_03_enum_derive_validate as assertions;

#[test]
fn stays_quiet_for_validated_enum_boundary() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/input.rs",
            "use garde::Validate;\nuse serde::Deserialize;\n\n#[derive(Deserialize, Validate)]\nenum Input {\n    Variant(String),\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}
