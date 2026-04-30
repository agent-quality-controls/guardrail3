use g3ts_style_config_checks_assertions as assertions;
use guardrail3_check_types::G3Severity;

#[test]
fn golden_style_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "g3ts-style/strict-policy-configured",
            "g3ts-style/style-packages-present",
            "g3ts-style/stylelint-config-present",
            "g3ts-style/stylelint-config-stack",
            "g3ts-style/stylelint-a11y-rules",
            "g3ts-style/css-lint-script",
            "g3ts-style/tailwind-ban-eslint-rule",
        ],
    );
}

#[test]
fn strict_policy_requires_source_globs() {
    let mut input = super::helpers::golden();
    let g3ts_style_types::G3TsStylePolicySurfaceState::Parsed { snapshot } =
        &mut input.contracts[0].policy
    else {
        panic!("golden policy should be parsed");
    };
    snapshot.source_globs.clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/strict-policy-configured",
        G3Severity::Error,
    );
}

#[test]
fn packages_must_be_directly_installed() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_package_mut(&mut input)
        .dev_dependencies
        .retain(|dependency| dependency != "stylelint-config-tailwindcss");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/style-packages-present",
        G3Severity::Error,
    );
}

#[test]
fn stylelint_config_must_be_present() {
    let mut input = super::helpers::golden();
    input.contracts[0].stylelint_config =
        g3ts_style_types::G3TsStylelintConfigSurfaceState::Missing {
            rel_path: "stylelint.config.*".to_owned(),
        };

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/stylelint-config-present",
        G3Severity::Error,
    );
}

#[test]
fn stylelint_stack_must_include_tailwind_and_a11y_plugin() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_stylelint_mut(&mut input)
        .raw_plugins
        .clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/stylelint-config-stack",
        G3Severity::Error,
    );
}

#[test]
fn stylelint_a11y_rules_must_be_effective_on_css_probe() {
    let mut input = super::helpers::golden();
    let snapshot = super::helpers::parsed_stylelint_mut(&mut input);
    snapshot
        .resolved_rule_names
        .retain(|rule| rule != "a11y/media-prefers-reduced-motion");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/stylelint-a11y-rules",
        G3Severity::Error,
    );
}

#[test]
fn css_lint_script_must_use_max_warnings_zero() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_package_mut(&mut input).script_tool_invocations[0]
        .args
        .retain(|arg| arg != "0");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/css-lint-script",
        G3Severity::Error,
    );
}

#[test]
fn css_lint_script_must_not_fail_open() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_package_mut(&mut input).script_tool_invocations[0].followed_by =
        Some(super::helpers::fail_open_separator());

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/css-lint-script",
        G3Severity::Error,
    );
}

#[test]
fn tailwind_ban_rule_must_be_effective_at_error_with_matching_denylist() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_eslint_mut(&mut input).tailwind_rule_effective = false;

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/tailwind-ban-eslint-rule",
        G3Severity::Error,
    );
}
