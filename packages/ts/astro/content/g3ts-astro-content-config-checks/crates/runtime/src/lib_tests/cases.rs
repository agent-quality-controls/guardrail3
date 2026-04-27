use g3ts_astro_content_config_checks_assertions::run as assertions;

#[test]
fn golden_content_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "TS-ASTRO-CONTENT-CONFIG-17",
            "TS-ASTRO-CONTENT-CONFIG-18",
            "TS-ASTRO-CONTENT-CONFIG-19",
            "TS-ASTRO-CONTENT-CONFIG-23",
            "TS-ASTRO-CONTENT-CONFIG-24",
            "TS-ASTRO-CONTENT-CONFIG-25",
            "TS-ASTRO-CONTENT-CONFIG-26",
            "TS-ASTRO-CONTENT-CONFIG-27",
            "TS-ASTRO-CONTENT-CONFIG-28",
        ],
    );
}
