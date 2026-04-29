use g3ts_astro_i18n_config_checks_assertions::run as assertions;

#[test]
fn golden_reports_i18n_inventory_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "g3ts-astro-i18n/strict-policy-configured",
            "g3ts-astro-i18n/policy-paths-valid",
            "g3ts-astro-i18n/i18next-plugin-package-present",
            "g3ts-astro-i18n/i18n-policy-plugin-package-present",
            "g3ts-astro-i18n/eslint-comments-plugin-package-present",
            "g3ts-astro-i18n/i18next-plugin-wired",
            "g3ts-astro-i18n/i18n-policy-plugin-wired",
            "g3ts-astro-i18n/no-unlocalized-internal-hrefs-rule",
            "g3ts-astro-i18n/no-inline-image-alt-rule",
            "g3ts-astro-i18n/require-content-image-key-rule",
            "g3ts-astro-i18n/raw-date-number-formatting-bans",
            "g3ts-astro-i18n/protected-i18n-rule-disables-restricted",
        ],
    );
}

#[test]
fn missing_i18n_policy_fails() {
    let mut input = super::helpers::golden();
    input.integration_contracts[0].astro_policy =
        g3ts_astro_i18n_types::G3TsAstroI18nPolicySurfaceState::MissingI18nPolicy {
            rel_path: "apps/landing/guardrail3-ts.toml".to_owned(),
        };

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-i18n/strict-policy-configured",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn missing_policy_rule_fails() {
    let mut input = super::helpers::golden();
    let g3ts_astro_i18n_types::G3TsAstroI18nEslintSurfaceState::Parsed { snapshot } =
        &mut input.eslint_contracts[0].config
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot
        .public_i18n_policy_rules
        .retain(|rule| rule != "astro-i18n-policy/no-inline-image-alt");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-i18n/no-inline-image-alt-rule",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn missing_i18next_rule_fails() {
    let mut input = super::helpers::golden();
    let g3ts_astro_i18n_types::G3TsAstroI18nEslintSurfaceState::Parsed { snapshot } =
        &mut input.eslint_contracts[0].config
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot
        .public_error_rules
        .retain(|rule| rule != "i18next/no-literal-string");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-i18n/i18next-plugin-wired",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn warning_level_formatting_bans_do_not_pass() {
    let mut input = super::helpers::golden();
    let g3ts_astro_i18n_types::G3TsAstroI18nEslintSurfaceState::Parsed { snapshot } =
        &mut input.eslint_contracts[0].config
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot.public_no_restricted_syntax_selectors.clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-i18n/raw-date-number-formatting-bans",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn helper_lane_must_not_inherit_raw_formatting_bans() {
    let mut input = super::helpers::golden();
    let g3ts_astro_i18n_types::G3TsAstroI18nEslintSurfaceState::Parsed { snapshot } =
        &mut input.eslint_contracts[0].config
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot.helper_no_restricted_syntax_selectors = vec![
        "CallExpression[callee.property.name='toLocaleDateString']".to_owned(),
    ];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-i18n/raw-date-number-formatting-bans",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn warning_level_disable_restriction_does_not_pass() {
    let mut input = super::helpers::golden();
    let g3ts_astro_i18n_types::G3TsAstroI18nEslintSurfaceState::Parsed { snapshot } =
        &mut input.eslint_contracts[0].config
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot.public_restricted_disable_patterns.clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-i18n/protected-i18n-rule-disables-restricted",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn missing_delegated_package_fails() {
    let mut input = super::helpers::golden();
    let g3ts_astro_i18n_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } =
        &mut input.integration_contracts[0].package
    else {
        unreachable!("test fixture must be parsed")
    };
    snapshot
        .dev_dependencies
        .retain(|package| package != "g3ts-eslint-plugin-astro-i18n-policy");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-i18n/i18n-policy-plugin-package-present",
        guardrail3_check_types::G3Severity::Error,
    );
}
