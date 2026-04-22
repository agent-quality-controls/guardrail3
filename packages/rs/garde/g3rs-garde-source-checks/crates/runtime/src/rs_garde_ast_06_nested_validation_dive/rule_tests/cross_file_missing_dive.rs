use g3rs_garde_source_checks_assertions::rs_garde_ast_06_nested_validation_dive::rule as assertions;

#[test]
fn errors_when_nested_validated_field_lacks_dive_across_files() {
    let fixture = super::helpers::fixture(vec![super::helpers::field(
        "src/input.rs",
        8,
        "Input",
        "payload",
        "Payload",
    )]);

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/input.rs"),
            title: Some("nested validated field `payload` missing garde(dive)"),
            message_contains: Some(
                "Nested validated fields must opt into recursive garde validation",
            ),
            ..Default::default()
        }],
    );
}
