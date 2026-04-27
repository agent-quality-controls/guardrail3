use g3ts_astro_mdx_config_checks_assertions::run as assertions;

#[test]
fn golden_mdx_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "TS-ASTRO-MDX-CONFIG-24",
            "TS-ASTRO-MDX-CONFIG-29",
            "TS-ASTRO-MDX-CONFIG-20",
            "TS-ASTRO-MDX-CONFIG-30",
            "TS-ASTRO-MDX-CONFIG-35",
            "TS-ASTRO-MDX-CONFIG-36",
            "TS-ASTRO-MDX-CONFIG-37",
            "TS-ASTRO-MDX-CONFIG-38",
        ],
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
        "TS-ASTRO-MDX-CONFIG-20",
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
        "TS-ASTRO-MDX-CONFIG-35",
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
        "TS-ASTRO-MDX-CONFIG-36",
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
        "TS-ASTRO-MDX-CONFIG-37",
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
        "TS-ASTRO-MDX-CONFIG-38",
        guardrail3_check_types::G3Severity::Error,
    );
}
