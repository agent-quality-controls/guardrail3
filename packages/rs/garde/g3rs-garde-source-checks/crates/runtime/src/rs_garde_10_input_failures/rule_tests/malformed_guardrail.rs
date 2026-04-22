use g3rs_garde_source_checks_assertions::rs_garde_10_input_failures::rule as assertions;

#[test]
fn reports_malformed_guardrail_config() {
    let fixture = super::helpers::fixture(vec![super::helpers::failure(
        "guardrail3-rs.toml",
        "Failed to parse `guardrail3-rs.toml` for garde Rust policy resolution: invalid guardrail3-rs.toml",
    )]);

    let results = fixture.run();

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("guardrail3-rs.toml"),
            title: Some("garde-family input failure"),
            ..Default::default()
        }],
    );
}
