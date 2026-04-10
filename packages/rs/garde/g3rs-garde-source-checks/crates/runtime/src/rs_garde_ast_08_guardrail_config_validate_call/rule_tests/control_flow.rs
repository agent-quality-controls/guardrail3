use g3rs_garde_source_checks_assertions::rs_garde_ast_08_guardrail_config_validate_call as assertions;

#[test]
fn errors_on_parse_inside_loop_without_validate() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    loop {\n        let config: GuardrailConfig = toml::from_str(content).ok()?;\n        break Some(config);\n    }\n}\n",
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
            ..Default::default()
        }],
    );
}

#[test]
fn errors_on_parse_inside_closure_without_validate() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    let build = || toml::from_str(content).ok();\n    build()\n}\n",
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
            ..Default::default()
        }],
    );
}

#[test]
fn errors_on_parse_inside_if_branch_without_validate() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\nfn load_config(content: &str, enabled: bool) -> Option<GuardrailConfig> {\n    if enabled {\n        return toml::from_str(content).ok();\n    }\n    None\n}\n",
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
            ..Default::default()
        }],
    );
}

#[test]
fn errors_on_parse_inside_match_arm_without_validate() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/load_config.rs",
            "use guardrail3_domain_config::types::GuardrailConfig;\n\nfn load_config(content: &str, enabled: bool) -> Option<GuardrailConfig> {\n    match enabled {\n        true => toml::from_str(content).ok(),\n        false => None,\n    }\n}\n",
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
            ..Default::default()
        }],
    );
}
