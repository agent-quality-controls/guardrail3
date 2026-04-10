use g3rs_garde_source_checks_assertions::rs_garde_ast_08_guardrail_config_validate_call as assertions;

#[test]
fn stays_quiet_when_guardrail_config_is_validated() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    let config: GuardrailConfig = toml::from_str(content).ok()?;\n    config.validate().ok()?;\n    Some(config)\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}

#[test]
fn stays_quiet_when_toml_from_str_is_validated_inline() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\nfn load_config(content: &str) -> Option<()> {\n    toml::from_str::<GuardrailConfig>(content).ok()?.validate().ok()?;\n    Some(())\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}

#[test]
fn stays_quiet_when_explicit_try_into_is_validated() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\nfn load_config(value: toml::Value) -> Option<GuardrailConfig> {\n    let config = value.try_into::<GuardrailConfig>().ok()?;\n    config.validate().ok()?;\n    Some(config)\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}
