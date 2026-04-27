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
