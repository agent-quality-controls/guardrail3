use g3ts_astro_seo_config_checks_assertions::run as assertions;

#[test]
fn golden_seo_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "TS-ASTRO-SEO-CONFIG-13",
            "TS-ASTRO-SEO-CONFIG-14",
            "TS-ASTRO-SEO-CONFIG-15",
            "TS-ASTRO-SEO-CONFIG-16",
            "TS-ASTRO-SEO-CONFIG-17",
            "TS-ASTRO-SEO-CONFIG-22",
            "TS-ASTRO-SEO-CONFIG-24",
            "TS-ASTRO-SEO-CONFIG-29",
            "TS-ASTRO-SEO-CONFIG-31",
            "TS-ASTRO-SEO-CONFIG-32",
        ],
    );
}
