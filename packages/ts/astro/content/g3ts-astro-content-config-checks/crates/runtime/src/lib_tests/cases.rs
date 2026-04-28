use g3ts_astro_content_config_checks_assertions::run as assertions;

#[test]
fn golden_content_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "g3ts-astro-content/pipeline-plugin-package-present",
            "g3ts-astro-content/content-adapter-rule",
            "g3ts-astro-content/inline-copy-rule",
            "g3ts-astro-content/protected-content-rule-disables-restricted",
            "g3ts-astro-content/strict-content-policy",
            "g3ts-astro-content/strict-policy-paths",
            "g3ts-astro-content/route-scope-overlap",
            "g3ts-astro-content/policy-eslint-coverage",
            "g3ts-astro-content/content-adapter-exists",
            "g3ts-astro-content/content-adapter-astro-content",
            "g3ts-astro-content/eslint-disable-inventory",
        ],
    );
}

#[test]
fn protected_content_rule_disables_must_cover_every_protected_rule() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_content_types::G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden content eslint config should be parsed");
    };
    snapshot
        .tsx_source_restricted_disable_patterns
        .retain(|rule| rule != "i18next/no-literal-string");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-content/protected-content-rule-disables-restricted",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn protected_content_rule_disables_requires_restrict_rule() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_content_types::G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden content eslint config should be parsed");
    };
    snapshot
        .astro_source_warn_or_error_rules
        .retain(|rule| rule != "@eslint-community/eslint-comments/no-restricted-disable");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-content/protected-content-rule-disables-restricted",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn protected_content_rule_disables_accept_pipeline_wildcard() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_content_types::G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden content eslint config should be parsed");
    };
    let patterns = vec![
        "astro-pipeline/*".to_owned(),
        "i18next/no-literal-string".to_owned(),
    ];
    snapshot.astro_source_restricted_disable_patterns = patterns.clone();
    snapshot.ts_source_restricted_disable_patterns = patterns.clone();
    snapshot.tsx_source_restricted_disable_patterns = patterns;

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-content/protected-content-rule-disables-restricted",
        guardrail3_check_types::G3Severity::Info,
    );
}

#[test]
fn eslint_disable_inventory_warns_when_content_source_contains_disable() {
    let mut input = super::helpers::golden();
    input.eslint_directives.push(
        g3ts_astro_content_types::G3TsAstroContentEslintDirectiveInput {
            rel_path: "src/pages/index.astro".to_owned(),
            directive_kind: "DisableNextLine".to_owned(),
            disabled_rules: vec!["i18next/no-literal-string".to_owned()],
            all_rules: false,
            line: 12,
            target_line: Some(13),
            parse_error: None,
        },
    );

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-content/eslint-disable-inventory",
        guardrail3_check_types::G3Severity::Warn,
    );
}

#[test]
fn eslint_disable_inventory_fails_closed_on_parse_error() {
    let mut input = super::helpers::golden();
    input.eslint_directives.push(
        g3ts_astro_content_types::G3TsAstroContentEslintDirectiveInput {
            rel_path: "src/pages/index.astro".to_owned(),
            directive_kind: "ParseError".to_owned(),
            disabled_rules: Vec::new(),
            all_rules: false,
            line: 0,
            target_line: None,
            parse_error: Some("ambiguous directive syntax".to_owned()),
        },
    );

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-content/eslint-disable-inventory",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn content_adapter_rule_accepts_equivalent_recursive_file_globs() {
    let mut input = super::helpers::golden();
    let config = &mut input.policy_eslint_contracts[0].eslint_config;
    let g3ts_astro_content_types::G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden content eslint config should be parsed");
    };
    let glob = "src/lib/content/**/*".to_owned();
    snapshot.astro_source_effective_content_adapter_modules = vec![glob.clone()];
    snapshot.ts_source_effective_content_adapter_modules = vec![glob.clone()];
    snapshot.tsx_source_effective_content_adapter_modules = vec![glob];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-content/content-adapter-rule",
        guardrail3_check_types::G3Severity::Info,
    );
}

#[test]
fn content_adapter_rule_rejects_unrelated_recursive_file_globs() {
    let mut input = super::helpers::golden();
    let config = &mut input.policy_eslint_contracts[0].eslint_config;
    let g3ts_astro_content_types::G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden content eslint config should be parsed");
    };
    let glob = "src/other-content/**/*".to_owned();
    snapshot.astro_source_effective_content_adapter_modules = vec![glob.clone()];
    snapshot.ts_source_effective_content_adapter_modules = vec![glob.clone()];
    snapshot.tsx_source_effective_content_adapter_modules = vec![glob];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-content/content-adapter-rule",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn content_adapter_rule_rejects_missing_tsx_lane_coverage() {
    let mut input = super::helpers::golden();
    let config = &mut input.policy_eslint_contracts[0].eslint_config;
    let g3ts_astro_content_types::G3TsAstroContentEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden content eslint config should be parsed");
    };
    snapshot
        .tsx_source_effective_content_adapter_modules
        .clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-content/content-adapter-rule",
        guardrail3_check_types::G3Severity::Error,
    );
}
