use g3_garde_ast_checks_assertions::rs_garde_14_guardrail_config_validate_call as assertions;

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
