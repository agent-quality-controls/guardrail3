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
