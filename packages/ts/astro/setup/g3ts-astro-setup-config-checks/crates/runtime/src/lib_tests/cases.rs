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
            "TS-ASTRO-SETUP-CONFIG-06",
            "TS-ASTRO-SETUP-CONFIG-07",
            "TS-ASTRO-SETUP-CONFIG-09",
            "TS-ASTRO-SETUP-CONFIG-10",
            "TS-ASTRO-SETUP-CONFIG-11",
            "TS-ASTRO-SETUP-CONFIG-12",
            "TS-ASTRO-SETUP-CONFIG-21",
        ],
    );
}
