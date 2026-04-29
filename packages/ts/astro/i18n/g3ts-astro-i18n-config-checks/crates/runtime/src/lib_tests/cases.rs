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
