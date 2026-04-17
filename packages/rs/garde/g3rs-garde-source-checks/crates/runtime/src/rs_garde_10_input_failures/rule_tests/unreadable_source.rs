use g3rs_garde_source_checks_assertions::rs_garde_10_input_failures::rule as assertions;

#[test]
fn reports_unreadable_rust_source() {
    let fixture = super::helpers::fixture(
        &[("src/lib.rs", "fn ok() {}\n")],
        super::helpers::default_guardrail_toml(),
    );
    fixture.make_source_unreadable("src/lib.rs");

    let results = fixture.run();

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/lib.rs"),
            title: Some("garde-family input failure"),
            ..Default::default()
        }],
    );
}
