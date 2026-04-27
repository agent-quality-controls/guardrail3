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
