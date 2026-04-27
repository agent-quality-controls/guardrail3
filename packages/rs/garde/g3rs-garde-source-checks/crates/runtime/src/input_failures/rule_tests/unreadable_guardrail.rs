use g3rs_garde_source_checks_assertions::input_failures::rule as assertions;

#[test]
fn reports_unreadable_guardrail_config() {
    let fixture = super::helpers::fixture(vec![super::helpers::failure(
        "guardrail3-rs.toml",
        "Failed to read `guardrail3-rs.toml` for garde Rust policy resolution: file is not readable",
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
