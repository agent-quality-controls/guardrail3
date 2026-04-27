use g3ts_astro_setup_config_checks_assertions::run as assertions;

#[test]
fn golden_setup_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "TS-ASTRO-SETUP-CONFIG-01",
            "TS-ASTRO-SETUP-CONFIG-02",
            "TS-ASTRO-SETUP-CONFIG-03",
            "TS-ASTRO-SETUP-CONFIG-05",
            "TS-ASTRO-SETUP-CONFIG-09",
            "TS-ASTRO-SETUP-CONFIG-10",
            "TS-ASTRO-SETUP-CONFIG-11",
            "TS-ASTRO-SETUP-CONFIG-12",
            "TS-ASTRO-SETUP-CONFIG-21",
        ],
    );
}

#[test]
fn astro_plugin_wiring_rejects_missing_effective_package_identity() {
    let mut input = super::helpers::golden();
    let config = &mut input.eslint_contracts[0].config;
    let g3ts_astro_setup_types::G3TsAstroSetupEslintSurfaceState::Parsed { snapshot } = config
    else {
        panic!("golden setup eslint config should be parsed");
    };
    snapshot.astro_source_plugin_package_names.clear();

    assertions::assert_runtime_check_id_severity(
        &input,
        "TS-ASTRO-SETUP-CONFIG-05",
        guardrail3_check_types::G3Severity::Error,
    );
}
