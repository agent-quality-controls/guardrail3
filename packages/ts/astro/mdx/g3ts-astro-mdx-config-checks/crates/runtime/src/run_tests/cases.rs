use g3ts_astro_mdx_config_checks_assertions::run as assertions;

#[test]
fn golden_mdx_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "g3ts-astro-mdx/strict-policy-paths",
            "g3ts-astro-mdx/policy-helper-surfaces",
            "g3ts-astro-mdx/mdx-eslint-plugin-package-present",
            "g3ts-astro-mdx/mdx-eslint-lane-wired",
            "g3ts-astro-mdx/mdx-component-map-rule",
            "g3ts-astro-mdx/mdx-import-names",
            "g3ts-astro-mdx/no-raw-ui-exports",
            "g3ts-astro-mdx/mdx-component-wrapper-zod-parse",
            "g3ts-astro-mdx/no-raw-mdx-images",
            "g3ts-astro-mdx/eslint-disable-descriptions-required",
            "g3ts-astro-mdx/unused-eslint-disables-fail",
            "g3ts-astro-mdx/protected-mdx-rule-disables-restricted",
            "g3ts-astro-mdx/eslint-disable-inventory",
        ],
    );
}

#[test]
fn mdx_package_rejects_missing_dependency() {
    let mut input = super::helpers::golden();
    let package = &mut input.integration_contracts[0].package;
    let g3ts_astro_mdx_types::G3TsAstroPackageSurfaceState::Parsed { snapshot } = package else {
        panic!("golden package should be parsed");
    };
    snapshot.dev_dependencies.clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/mdx-eslint-plugin-package-present",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn mdx_lane_rejects_missing_effective_package_identity() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_mdx_types::G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden mdx eslint config should be parsed");
    };
    snapshot.mdx_content_plugin_package_names.clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/mdx-eslint-lane-wired",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn mdx_import_name_rule_is_required() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_mdx_types::G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden mdx eslint config should be parsed");
    };
    snapshot
        .mdx_content_effective_named_component_import_rules
        .clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/mdx-import-names",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn mdx_component_map_raw_ui_export_rule_is_required() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_mdx_types::G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden mdx eslint config should be parsed");
    };
    snapshot
        .component_map_effective_no_raw_ui_export_rules
        .clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/no-raw-ui-exports",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn mdx_component_wrapper_zod_parse_rule_is_required() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_mdx_types::G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden mdx eslint config should be parsed");
    };
    snapshot
        .component_map_effective_wrapper_zod_parse_rules
        .clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/mdx-component-wrapper-zod-parse",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn mdx_raw_image_rule_is_required() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_mdx_types::G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden mdx eslint config should be parsed");
    };
    snapshot.mdx_content_effective_no_raw_image_rules.clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/no-raw-mdx-images",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn protected_mdx_rule_disables_must_cover_component_map_rules() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_mdx_types::G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden mdx eslint config should be parsed");
    };
    snapshot
        .component_map_restricted_disable_patterns
        .retain(|rule| rule != "astro-pipeline/mdx-component-wrapper-requires-zod-parse");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/protected-mdx-rule-disables-restricted",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn protected_mdx_rule_disables_requires_restrict_rule() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_mdx_types::G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden mdx eslint config should be parsed");
    };
    snapshot
        .mdx_content_warn_or_error_rules
        .retain(|rule| rule != "@eslint-community/eslint-comments/no-restricted-disable");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/protected-mdx-rule-disables-restricted",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn protected_mdx_rule_disables_accept_pipeline_wildcard() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_mdx_types::G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden mdx eslint config should be parsed");
    };
    snapshot.mdx_content_restricted_disable_patterns =
        vec!["mdx/remark".to_owned(), "astro-pipeline/*".to_owned()];
    snapshot.component_map_restricted_disable_patterns = vec!["astro-pipeline/*".to_owned()];

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/protected-mdx-rule-disables-restricted",
        guardrail3_check_types::G3Severity::Info,
    );
}

#[test]
fn mdx_disable_descriptions_must_be_error_on_mdx_lanes() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_mdx_types::G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden mdx eslint config should be parsed");
    };
    snapshot
        .mdx_content_error_rules
        .retain(|rule| rule != "@eslint-community/eslint-comments/require-description");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/eslint-disable-descriptions-required",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn mdx_disable_descriptions_accepts_namespace_when_package_identity_is_unavailable() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_mdx_types::G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden mdx eslint config should be parsed");
    };
    snapshot.mdx_content_plugin_package_names.clear();
    snapshot.component_map_plugin_package_names.clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/eslint-disable-descriptions-required",
        guardrail3_check_types::G3Severity::Info,
    );
}

#[test]
fn mdx_disable_descriptions_rejects_missing_plugin_namespace() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_mdx_types::G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden mdx eslint config should be parsed");
    };
    snapshot
        .component_map_plugins
        .retain(|plugin| plugin != "@eslint-community/eslint-comments");

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/eslint-disable-descriptions-required",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn mdx_unused_disables_must_fail_closed_on_mdx_lanes() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_mdx_types::G3TsAstroMdxEslintSurfaceState::Parsed { snapshot } = config else {
        panic!("golden mdx eslint config should be parsed");
    };
    snapshot.component_map_unused_disable_fail_closed = false;

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/unused-eslint-disables-fail",
        guardrail3_check_types::G3Severity::Error,
    );
}

#[test]
fn eslint_disable_inventory_warns_when_mdx_source_contains_disable() {
    let mut input = super::helpers::golden();
    input
        .eslint_directives
        .push(g3ts_astro_mdx_types::G3TsAstroMdxEslintDirectiveInput {
            rel_path: "content/blog/post.mdx".to_owned(),
            directive_kind: "DisableNextLine".to_owned(),
            disabled_rules: vec!["mdx/remark".to_owned()],
            all_rules: false,
            line: 5,
            target_line: Some(6),
            parse_error: None,
        });

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/eslint-disable-inventory",
        guardrail3_check_types::G3Severity::Warn,
    );
}

#[test]
fn eslint_disable_inventory_fails_closed_on_mdx_parse_error() {
    let mut input = super::helpers::golden();
    input
        .eslint_directives
        .push(g3ts_astro_mdx_types::G3TsAstroMdxEslintDirectiveInput {
            rel_path: "content/blog/post.mdx".to_owned(),
            directive_kind: "ParseError".to_owned(),
            disabled_rules: Vec::new(),
            all_rules: false,
            line: 0,
            target_line: None,
            parse_error: Some("ambiguous MDX directive syntax".to_owned()),
        });

    assertions::assert_runtime_check_id_severity(
        &input,
        "g3ts-astro-mdx/eslint-disable-inventory",
        guardrail3_check_types::G3Severity::Error,
    );
}
