use g3rs_garde_source_checks_assertions::manual_deserialize_impl::rule as assertions;

#[test]
fn errors_on_manual_deserialize_without_validate() {
    let fixture = super::helpers::fixture(vec![super::helpers::target(
        "src/input.rs",
        6,
        "Input",
        true,
        false,
    )]);

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/input.rs"),
            title: Some("manual Deserialize impl for `Input` without Validate"),
            message_contains: Some("bypasses derive-based garde checks"),
            ..Default::default()
        }],
    );
}
