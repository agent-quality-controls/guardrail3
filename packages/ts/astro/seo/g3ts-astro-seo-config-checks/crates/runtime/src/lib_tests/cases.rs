use g3ts_astro_seo_config_checks_assertions::run as assertions;

#[test]
fn golden_seo_package_reports_owned_ids() {
    assertions::assert_runtime_check_exact_ids(
        &super::helpers::golden(),
        &[
            "g3ts-astro-seo/nuasite-checks",
            "g3ts-astro-seo/sitemap-integration",
            "g3ts-astro-seo/robots-integration",
            "g3ts-astro-seo/llms-txt",
            "g3ts-astro-seo/seo-packages",
            "g3ts-astro-seo/structured-data-check",
            "g3ts-astro-seo/strict-policy-paths",
            "g3ts-astro-seo/policy-helper-surfaces",
            "g3ts-astro-seo/metadata-helper-rule",
            "g3ts-astro-seo/json-ld-helper-rule",
        ],
    );
}
