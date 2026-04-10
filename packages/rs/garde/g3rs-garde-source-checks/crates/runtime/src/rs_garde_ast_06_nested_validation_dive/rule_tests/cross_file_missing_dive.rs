use g3rs_garde_source_checks_assertions::rs_garde_ast_06_nested_validation_dive as assertions;

#[test]
fn errors_when_nested_validated_field_lacks_dive_across_files() {
    let fixture = crate::test_support::fixture(
        &[
            (
                "src/payload.rs",
                "use garde::Validate;\nuse serde::Deserialize;\n\n#[derive(Deserialize, Validate)]\npub struct Payload {\n    #[garde(length(min = 1))]\n    title: String,\n}\n",
            ),
            (
                "src/input.rs",
                "use garde::Validate;\nuse serde::Deserialize;\n\nuse crate::payload::Payload;\n\n#[derive(Deserialize, Validate)]\nstruct Input {\n    payload: Payload,\n}\n",
            ),
        ],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/input.rs"),
            title: Some("nested validated field `payload` missing garde(dive)"),
            message_contains: Some("Nested validated fields must opt into recursive garde validation"),
            ..Default::default()
        }],
    );
}
