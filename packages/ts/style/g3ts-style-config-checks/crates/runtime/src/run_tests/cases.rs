use g3ts_style_config_checks_assertions as assertions;
use guardrail3_check_types::G3Severity;

#[test]
fn golden_style_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "g3ts-style/strict-policy-configured",
            "g3ts-style/policy-paths-valid",
            "g3ts-style/style-packages-present",
            "g3ts-style/stylelint-config-present",
            "g3ts-style/stylelint-config-stack",
            "g3ts-style/stylelint-a11y-rules",
            "g3ts-style/css-lint-script",
            "g3ts-style/validate-runs-css-lint",
            "g3ts-style/style-policy-eslint-rule",
            "g3ts-style/protected-style-rule-disables-restricted",
            "g3ts-style/syncpack-style-policy-pin",
            "g3ts-style/eslint-disable-inventory",
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
    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Info,
    );
}

#[test]
fn strict_policy_requires_stylelint_css_globs() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_policy_mut(&mut input)
        .stylelint_css_globs
        .clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/strict-policy-configured",
        G3Severity::Error,
    );
    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Info,
    );
}

#[test]
fn policy_paths_reject_empty_source_glob_value() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_policy_mut(&mut input).source_globs = vec![String::new()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/strict-policy-configured",
        G3Severity::Info,
    );
    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Error,
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/policy-paths-valid",
        "source_globs=``",
    );
}

#[test]
fn policy_paths_reject_empty_css_glob_value() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_policy_mut(&mut input).stylelint_css_globs = vec![String::new()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/strict-policy-configured",
        G3Severity::Info,
    );
    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Error,
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/policy-paths-valid",
        "stylelint_css_globs=``",
    );
}

#[test]
fn policy_paths_reject_absolute_source_glob() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_policy_mut(&mut input).source_globs = vec!["/src/**/*.tsx".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/strict-policy-configured",
        G3Severity::Info,
    );
    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Error,
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/policy-paths-valid",
        "source_globs=`/src/**/*.tsx`",
    );
}

#[test]
fn policy_paths_reject_absolute_css_glob() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_policy_mut(&mut input).stylelint_css_globs =
        vec!["/src/**/*.css".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Error,
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/policy-paths-valid",
        "stylelint_css_globs=`/src/**/*.css`",
    );
}

#[test]
fn policy_paths_reject_parent_traversal_source_glob() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_policy_mut(&mut input).source_globs =
        vec!["../shared/**/*.tsx".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Error,
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/policy-paths-valid",
        "source_globs=`../shared/**/*.tsx`",
    );
}

#[test]
fn policy_paths_reject_parent_traversal_css_glob() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_policy_mut(&mut input).stylelint_css_globs =
        vec!["../shared/**/*.css".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Error,
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/policy-paths-valid",
        "stylelint_css_globs=`../shared/**/*.css`",
    );
}

#[test]
fn policy_paths_reject_external_url_source_glob() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_policy_mut(&mut input).source_globs =
        vec!["https://example.com/source.tsx".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Error,
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/policy-paths-valid",
        "source_globs=`https://example.com/source.tsx`",
    );
}

#[test]
fn policy_paths_reject_external_url_css_glob() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_policy_mut(&mut input).stylelint_css_globs =
        vec!["https://example.com/styles.css".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Error,
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/policy-paths-valid",
        "stylelint_css_globs=`https://example.com/styles.css`",
    );
}

#[test]
fn policy_paths_reject_scheme_without_slashes() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_policy_mut(&mut input).source_globs =
        vec!["data:text/css,body{}".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Error,
    );
}

#[test]
fn policy_paths_reject_windows_absolute_path() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_policy_mut(&mut input).source_globs =
        vec!["C:\\repo\\src\\page.tsx".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Error,
    );
}

#[test]
fn policy_paths_reject_backslash_traversal() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_policy_mut(&mut input).stylelint_css_globs =
        vec!["..\\shared\\style.css".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Error,
    );
}

#[test]
fn policy_paths_reject_encoded_traversal_and_separators() {
    let mut input = super::helpers::golden();
    let policy = super::helpers::parsed_policy_mut(&mut input);
    policy.source_globs = vec!["src/%2e%2e/secret.tsx".to_owned()];
    policy.stylelint_css_globs = vec!["src%2f..%2fsecret.css".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Error,
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/policy-paths-valid",
        "source_globs=`src/%2e%2e/secret.tsx`",
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/policy-paths-valid",
        "stylelint_css_globs=`src%2f..%2fsecret.css`",
    );
}

#[test]
fn policy_paths_report_multiple_invalid_values() {
    let mut input = super::helpers::golden();
    let policy = super::helpers::parsed_policy_mut(&mut input);
    policy.source_globs = vec![
        "/src/**/*.tsx".to_owned(),
        "data:text/css,body{}".to_owned(),
    ];
    policy.stylelint_css_globs = vec!["../shared/**/*.css".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/policy-paths-valid",
        G3Severity::Error,
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/policy-paths-valid",
        "source_globs=`/src/**/*.tsx`",
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/policy-paths-valid",
        "source_globs=`data:text/css,body{}`",
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/policy-paths-valid",
        "stylelint_css_globs=`../shared/**/*.css`",
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
fn validate_script_must_reach_css_lint() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_package_mut(&mut input)
        .script_tool_invocations
        .retain(|invocation| invocation.script_name != "validate");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/validate-runs-css-lint",
        G3Severity::Error,
    );
}

#[test]
fn validate_script_must_not_fail_open() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package
        .script_tool_invocations
        .iter_mut()
        .filter(|invocation| invocation.script_name == "validate")
        .for_each(|invocation| {
            invocation.followed_by = Some(super::helpers::fail_open_separator());
        });

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/validate-runs-css-lint",
        G3Severity::Error,
    );
}

#[test]
fn validate_script_package_parse_failures_fail_closed() {
    let mut input = super::helpers::golden();
    input.contracts[0].package = g3ts_style_types::G3TsStylePackageSurfaceState::ParseError {
        rel_path: "package.json".to_owned(),
        reason: "broken package json".to_owned(),
    };

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/validate-runs-css-lint",
        G3Severity::Error,
    );
}

#[test]
fn validate_script_rejects_reachable_parse_blockers() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_package_mut(&mut input)
        .script_parse_blockers
        .push(g3ts_style_types::G3TsStylePackageScriptParseBlocker {
            script_name: "lint:css".to_owned(),
            reason: "unsupported shell".to_owned(),
        });

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/validate-runs-css-lint",
        G3Severity::Error,
    );
}

#[test]
fn validate_script_accepts_package_manager_script_shorthand() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package
        .script_tool_invocations
        .retain(|invocation| invocation.script_name != "validate");
    package
        .script_tool_invocations
        .push(g3ts_style_types::G3TsStylePackageScriptToolInvocation {
            script_name: "validate".to_owned(),
            executable: "lint:css".to_owned(),
            args: Vec::new(),
            preceded_by: None,
            followed_by: None,
        });

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/validate-runs-css-lint",
        G3Severity::Info,
    );
}

#[test]
fn validate_script_accepts_direct_package_manager_stylelint() {
    let mut input = super::helpers::golden();
    let package = super::helpers::parsed_package_mut(&mut input);
    package
        .script_tool_invocations
        .retain(|invocation| invocation.script_name != "validate");
    package
        .script_tool_invocations
        .push(g3ts_style_types::G3TsStylePackageScriptToolInvocation {
            script_name: "validate".to_owned(),
            executable: "stylelint".to_owned(),
            args: vec![
                "--max-warnings".to_owned(),
                "0".to_owned(),
            ],
            preceded_by: None,
            followed_by: None,
        });

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/validate-runs-css-lint",
        G3Severity::Info,
    );
}

#[test]
fn style_policy_rule_must_be_effective_at_error_with_non_empty_eslint_policy() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_eslint_mut(&mut input).style_policy_rule_effective = false;

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/style-policy-eslint-rule",
        G3Severity::Error,
    );
}

#[test]
fn style_policy_rule_must_use_owned_plugin_package_on_every_probe() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_eslint_mut(&mut input).style_policy_plugin_effective = false;

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/style-policy-eslint-rule",
        G3Severity::Error,
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/style-policy-eslint-rule",
        "g3ts-eslint-plugin-style-policy",
    );
}

#[test]
fn protected_style_rule_disables_must_restrict_style_rules() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_eslint_mut(&mut input)
        .source_probe_disable_policies[0]
        .restricted_disable_patterns
        .retain(|pattern| pattern != "style-policy/*");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/protected-style-rule-disables-restricted",
        G3Severity::Error,
    );
}

#[test]
fn protected_style_rule_disables_accept_wildcard() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_eslint_mut(&mut input).source_probe_disable_policies[0]
        .restricted_disable_patterns = vec!["*".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/protected-style-rule-disables-restricted",
        G3Severity::Info,
    );
}

#[test]
fn protected_style_rule_disables_must_be_per_probe() {
    let mut input = super::helpers::golden();
    let protected_probe = super::helpers::parsed_eslint_mut(&mut input)
        .source_probe_disable_policies[0]
        .clone();
    let mut unprotected_probe = protected_probe.clone();
    unprotected_probe.rel_path = "src/__g3ts_style_probe__.astro".to_owned();
    unprotected_probe.restricted_disable_patterns = vec!["style-policy/*".to_owned()];
    super::helpers::parsed_eslint_mut(&mut input)
        .source_probe_disable_policies
        .push(unprotected_probe);

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/protected-style-rule-disables-restricted",
        G3Severity::Error,
    );
}

#[test]
fn eslint_disable_inventory_reports_visible_warning() {
    let mut input = super::helpers::golden();
    input
        .eslint_directives
        .push(g3ts_style_types::G3TsStyleEslintDirectiveInput {
            rel_path: "src/page.tsx".to_owned(),
            directive_kind: "DisableNextLine".to_owned(),
            disabled_rules: vec!["style-policy/no-denied-class-tokens".to_owned()],
            all_rules: false,
            line: 12,
            target_line: Some(13),
            parse_error: None,
        });

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/eslint-disable-inventory",
        G3Severity::Warn,
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/eslint-disable-inventory",
        "style-policy/no-denied-class-tokens",
    );
}

#[test]
fn eslint_disable_inventory_parse_errors_fail_closed() {
    let mut input = super::helpers::golden();
    input
        .eslint_directives
        .push(g3ts_style_types::G3TsStyleEslintDirectiveInput {
            rel_path: "src/page.tsx".to_owned(),
            directive_kind: String::new(),
            disabled_rules: Vec::new(),
            all_rules: false,
            line: 0,
            target_line: None,
            parse_error: Some("broken source".to_owned()),
        });

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/eslint-disable-inventory",
        G3Severity::Error,
    );
}

#[test]
fn syncpack_must_pin_style_policy_plugin_floor() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_syncpack_mut(&mut input)
        .version_groups
        .clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/syncpack-style-policy-pin",
        G3Severity::Error,
    );
}

#[test]
fn syncpack_must_reject_wrong_pin_version() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_syncpack_mut(&mut input).version_groups[0].pin_version =
        Some("0.1.2".to_owned());

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/syncpack-style-policy-pin",
        G3Severity::Error,
    );
}

#[test]
fn syncpack_must_reject_duplicate_pin_groups() {
    let mut input = super::helpers::golden();
    let duplicate = super::helpers::parsed_syncpack_mut(&mut input).version_groups[0].clone();
    super::helpers::parsed_syncpack_mut(&mut input)
        .version_groups
        .push(duplicate);

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/syncpack-style-policy-pin",
        G3Severity::Error,
    );
}

#[test]
fn syncpack_must_reject_package_scoped_pin_groups() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_syncpack_mut(&mut input).version_groups[0].packages =
        Some(vec!["landing".to_owned()]);

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/syncpack-style-policy-pin",
        G3Severity::Error,
    );
}

#[test]
fn syncpack_must_reject_ignored_pin_groups() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_syncpack_mut(&mut input).version_groups[0].is_ignored = Some(true);

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/syncpack-style-policy-pin",
        G3Severity::Error,
    );
}

#[test]
fn syncpack_must_cover_package_manifest() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_syncpack_mut(&mut input).source = vec!["packages/*/package.json".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/syncpack-style-policy-pin",
        G3Severity::Error,
    );
}

#[test]
fn style_packages_reject_legacy_tailwind_ban_plugin() {
    let mut input = super::helpers::golden();
    super::helpers::parsed_package_mut(&mut input)
        .dev_dependencies
        .push("eslint-plugin-tailwind-ban".to_owned());

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-style/style-packages-present",
        G3Severity::Error,
    );
    assertions::assert_runtime_check_message_contains(
        &input,
        "g3ts-style/style-packages-present",
        "remove `eslint-plugin-tailwind-ban`",
    );
}
