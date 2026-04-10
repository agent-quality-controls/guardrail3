use g3rs_garde_ast_checks_assertions::rs_garde_ast_08_guardrail_config_validate_call as assertions;

#[test]
fn stays_quiet_when_rebound_value_is_validated_before_use() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    let raw: GuardrailConfig = toml::from_str(content).ok()?;\n    let config = raw;\n    config.validate().ok()?;\n    Some(config)\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}

#[test]
fn stays_quiet_when_assignment_rebound_value_is_validated_before_use() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    let mut config: GuardrailConfig = toml::from_str(content).ok()?;\n    let rebound = config;\n    config = rebound;\n    config.validate().ok()?;\n    Some(config)\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}

#[test]
fn errors_when_rebound_value_is_used_before_validate() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    let raw: GuardrailConfig = toml::from_str(content).ok()?;\n    let config = raw;\n    let _ = &config;\n    config.validate().ok()?;\n    Some(config)\n}\n",
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
