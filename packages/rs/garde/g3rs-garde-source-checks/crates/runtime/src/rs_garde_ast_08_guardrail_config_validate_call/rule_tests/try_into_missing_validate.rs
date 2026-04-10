use g3rs_garde_source_checks_assertions::rs_garde_ast_08_guardrail_config_validate_call as assertions;

#[test]
fn errors_on_try_into_without_validate() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\nfn load_config(value: toml::Value) -> Option<GuardrailConfig> {\n    value.try_into().ok()\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/load_config.rs"),
            title: Some("`GuardrailConfig` parse without garde validation"),
            message_contains: Some("does not call `.validate()` on it"),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_on_explicit_try_into_without_validate() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\nfn load_config(value: toml::Value) -> Option<GuardrailConfig> {\n    value.try_into::<GuardrailConfig>().ok()\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/load_config.rs"),
            title: Some("`GuardrailConfig` parse without garde validation"),
            message_contains: Some("does not call `.validate()` on it"),
            ..Default::default()
        }],
    );
}
