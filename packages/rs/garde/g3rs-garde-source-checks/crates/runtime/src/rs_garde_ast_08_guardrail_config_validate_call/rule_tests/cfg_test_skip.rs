use g3rs_garde_source_checks_assertions::rs_garde_ast_08_guardrail_config_validate_call as assertions;

#[test]
fn skips_cfg_test_guardrail_parse_sites() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\n#[cfg(test)]\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    toml::from_str(content).ok()\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}

#[test]
fn skips_cfg_test_module_bodies() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n    fn load_config(content: &str) -> Option<GuardrailConfig> {\n        toml::from_str(content).ok()\n    }\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}

#[test]
fn skips_cfg_test_impl_methods() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\nstruct Loader;\n\nimpl Loader {\n    #[cfg(test)]\n    fn load_config(&self, content: &str) -> Option<GuardrailConfig> {\n        toml::from_str(content).ok()\n    }\n}\n",
        )],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}
