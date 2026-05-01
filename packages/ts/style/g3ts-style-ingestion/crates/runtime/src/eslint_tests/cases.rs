#[test]
fn style_policy_option_rejects_empty_policy() {
    g3ts_style_ingestion_assertions::eslint::assert_style_policy_option_rejected(
        super::super::option_has_non_empty_style_policy(&serde_json::json!({
            "denyList": []
        })),
    );
}

#[test]
fn style_policy_option_rejects_blank_items() {
    g3ts_style_ingestion_assertions::eslint::assert_style_policy_option_rejected(
        super::super::option_has_non_empty_style_policy(&serde_json::json!({
            "denyList": ["  "]
        })),
    );
}

#[test]
fn style_policy_option_accepts_exact_denylist() {
    g3ts_style_ingestion_assertions::eslint::assert_style_policy_option_accepted(
        super::super::option_has_non_empty_style_policy(&serde_json::json!({
            "denyList": ["text-slate-500", "prose"]
        })),
    );
}

#[test]
fn style_policy_option_accepts_non_empty_prefixes() {
    g3ts_style_ingestion_assertions::eslint::assert_style_policy_option_accepted(
        super::super::option_has_non_empty_style_policy(&serde_json::json!({
            "denyPrefixes": ["text-["]
        })),
    );
}

#[test]
fn style_policy_option_accepts_non_empty_patterns() {
    g3ts_style_ingestion_assertions::eslint::assert_style_policy_option_accepted(
        super::super::option_has_non_empty_style_policy(&serde_json::json!({
            "denyPatterns": ["^text-\\["]
        })),
    );
}

#[test]
fn style_policy_rule_uses_first_options_object_only() {
    let rule = eslint_config_parser::types::EslintRuleSetting {
        severity: eslint_config_parser::types::EslintRuleSeverity::Error,
        options: vec![
            serde_json::json!({}),
            serde_json::json!({ "denyList": ["text-black"] }),
        ],
    };

    assert!(
        !super::super::rule_has_effective_style_policy(&rule),
        "G3TS must not accept denyList from a later options object because ESLint rule runtime reads context.options[0]"
    );
}

#[test]
fn style_policy_rule_requires_error_severity() {
    let rule = eslint_config_parser::types::EslintRuleSetting {
        severity: eslint_config_parser::types::EslintRuleSeverity::Warn,
        options: vec![serde_json::json!({ "denyList": ["text-black"] })],
    };

    assert!(
        !super::super::rule_has_effective_style_policy(&rule),
        "style policy must fail closed at error severity"
    );
}

#[test]
fn style_policy_rule_accepts_first_options_object_policy() {
    let rule = eslint_config_parser::types::EslintRuleSetting {
        severity: eslint_config_parser::types::EslintRuleSeverity::Error,
        options: vec![serde_json::json!({ "denyList": ["text-black"] })],
    };

    assert!(
        super::super::rule_has_effective_style_policy(&rule),
        "first options object with a non-empty deny policy is the effective ESLint policy"
    );
}

#[test]
fn style_policy_rule_accepts_first_options_object_prefix_policy() {
    let rule = eslint_config_parser::types::EslintRuleSetting {
        severity: eslint_config_parser::types::EslintRuleSeverity::Error,
        options: vec![serde_json::json!({ "denyPrefixes": ["text-["] })],
    };

    assert!(
        super::super::rule_has_effective_style_policy(&rule),
        "first options object with a non-empty denyPrefixes policy is effective"
    );
}

#[test]
fn style_policy_rule_accepts_first_options_object_pattern_policy() {
    let rule = eslint_config_parser::types::EslintRuleSetting {
        severity: eslint_config_parser::types::EslintRuleSeverity::Error,
        options: vec![serde_json::json!({ "denyPatterns": ["^text-\\\\["] })],
    };

    assert!(
        super::super::rule_has_effective_style_policy(&rule),
        "first options object with a non-empty denyPatterns policy is effective"
    );
}

#[test]
fn style_policy_plugin_identity_must_match_on_every_probe() {
    let probes = vec![
        probe_with_style_policy_package("src/index.ts", "g3ts-eslint-plugin-style-policy"),
        probe_with_style_policy_package("src/component.tsx", "not-g3ts-style-plugin"),
    ];

    assert!(
        !super::super::all_probes_use_owned_style_policy_plugin(&probes),
        "aggregate namespace package identity is not enough; every source probe must use the owned style-policy plugin"
    );
}

#[test]
fn style_policy_plugin_identity_accepts_owned_package_on_every_probe() {
    let probes = vec![
        probe_with_style_policy_package("src/index.ts", "g3ts-eslint-plugin-style-policy"),
        probe_with_style_policy_package("src/component.tsx", "g3ts-eslint-plugin-style-policy"),
    ];

    assert!(
        super::super::all_probes_use_owned_style_policy_plugin(&probes),
        "every source probe uses g3ts-eslint-plugin-style-policy"
    );
}

fn probe_with_style_policy_package(
    rel_path: &str,
    package: &str,
) -> eslint_config_parser::types::EslintEffectiveConfigProbe {
    eslint_config_parser::types::EslintEffectiveConfigProbe {
        probe: eslint_config_parser::types::EslintProbeKind::TsxSource,
        rel_path: rel_path.to_owned(),
        ignored: false,
        plugins: vec!["style-policy".to_owned()],
        plugin_meta_names: BTreeMap::new(),
        plugin_package_names: BTreeMap::from([(
            "style-policy".to_owned(),
            vec![package.to_owned()],
        )]),
        rules: BTreeMap::new(),
        project_service: None,
        linter_options_no_inline_config: None,
        linter_options_report_unused_disable_directives: None,
        linter_options_report_unused_inline_configs: None,
    }
}
use std::collections::BTreeMap;
