use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3TsAstroConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    results.extend(check_setup(input));
    results.extend(check_content(input));
    results.extend(check_mdx(input));
    results.extend(check_seo(input));
    results
}

pub fn check_setup(input: &G3TsAstroConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_config_01_astro_package_present::check(input, &mut results);
    crate::ts_astro_config_02_astro_check_present::check(input, &mut results);
    crate::ts_astro_config_03_astro_eslint_plugin_package_present::check(input, &mut results);
    crate::ts_astro_config_05_astro_eslint_plugin_wired::check(input, &mut results);
    crate::ts_astro_config_06_pipeline_plugin_package_present::check(input, &mut results);
    crate::ts_astro_config_07_pipeline_plugin_wired::check(input, &mut results);
    crate::ts_astro_config_09_syncpack_stack_pins::check(input, &mut results);
    crate::ts_astro_config_10_syncpack_forbidden_deps::check(input, &mut results);
    crate::ts_astro_config_11_site_url::check(input, &mut results);
    crate::ts_astro_config_12_static_output::check(input, &mut results);
    crate::ts_astro_config_21_required_integrations::check(input, &mut results);
    remap_ids(
        &mut results,
        &[
            ("TS-ASTRO-CONFIG-01", "TS-ASTRO-SETUP-CONFIG-01"),
            ("TS-ASTRO-CONFIG-02", "TS-ASTRO-SETUP-CONFIG-02"),
            ("TS-ASTRO-CONFIG-03", "TS-ASTRO-SETUP-CONFIG-03"),
            ("TS-ASTRO-CONFIG-05", "TS-ASTRO-SETUP-CONFIG-05"),
            ("TS-ASTRO-CONFIG-06", "TS-ASTRO-SETUP-CONFIG-06"),
            ("TS-ASTRO-CONFIG-07", "TS-ASTRO-SETUP-CONFIG-07"),
            ("TS-ASTRO-CONFIG-09", "TS-ASTRO-SETUP-CONFIG-09"),
            ("TS-ASTRO-CONFIG-10", "TS-ASTRO-SETUP-CONFIG-10"),
            ("TS-ASTRO-CONFIG-11", "TS-ASTRO-SETUP-CONFIG-11"),
            ("TS-ASTRO-CONFIG-12", "TS-ASTRO-SETUP-CONFIG-12"),
            ("TS-ASTRO-CONFIG-21", "TS-ASTRO-SETUP-CONFIG-21"),
        ],
    );
    results
}

pub fn check_content(input: &G3TsAstroConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_config_18_content_adapter_rule::check(input, &mut results);
    crate::ts_astro_config_19_inline_copy_rule::check(input, &mut results);
    crate::ts_astro_config_23_strict_content_policy::check_content(input, &mut results);
    crate::ts_astro_config_24_strict_policy_paths::check_content(input, &mut results);
    crate::ts_astro_config_25_route_scope_overlap::check(input, &mut results);
    crate::ts_astro_config_26_policy_eslint_coverage::check(input, &mut results);
    crate::ts_astro_config_27_content_adapter_exists::check(input, &mut results);
    crate::ts_astro_config_28_content_adapter_astro_content::check(input, &mut results);
    remap_ids(
        &mut results,
        &[
            ("TS-ASTRO-CONFIG-18", "TS-ASTRO-CONTENT-CONFIG-18"),
            ("TS-ASTRO-CONFIG-19", "TS-ASTRO-CONTENT-CONFIG-19"),
            ("TS-ASTRO-CONFIG-25", "TS-ASTRO-CONTENT-CONFIG-25"),
            ("TS-ASTRO-CONFIG-26", "TS-ASTRO-CONTENT-CONFIG-26"),
            ("TS-ASTRO-CONFIG-27", "TS-ASTRO-CONTENT-CONFIG-27"),
            ("TS-ASTRO-CONFIG-28", "TS-ASTRO-CONTENT-CONFIG-28"),
        ],
    );
    results
}

pub fn check_mdx(input: &G3TsAstroConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_config_24_strict_policy_paths::check_mdx(input, &mut results);
    crate::ts_astro_config_29_policy_helper_surfaces::check_mdx(input, &mut results);
    crate::ts_astro_config_20_mdx_lane::check(input, &mut results);
    crate::ts_astro_config_30_mdx_component_map_rule::check(input, &mut results);
    remap_ids(
        &mut results,
        &[
            ("TS-ASTRO-CONFIG-20", "TS-ASTRO-MDX-CONFIG-20"),
            ("TS-ASTRO-CONFIG-30", "TS-ASTRO-MDX-CONFIG-30"),
        ],
    );
    results
}

pub fn check_seo(input: &G3TsAstroConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_config_13_nuasite_checks::check(input, &mut results);
    crate::ts_astro_config_14_sitemap_integration::check(input, &mut results);
    crate::ts_astro_config_15_robots_integration::check(input, &mut results);
    crate::ts_astro_config_16_llms_txt::check(input, &mut results);
    crate::ts_astro_config_17_seo_packages::check(input, &mut results);
    crate::ts_astro_config_22_structured_data_check::check(input, &mut results);
    crate::ts_astro_config_24_strict_policy_paths::check_seo(input, &mut results);
    crate::ts_astro_config_29_policy_helper_surfaces::check_seo(input, &mut results);
    crate::ts_astro_config_31_metadata_helper_rule::check(input, &mut results);
    crate::ts_astro_config_32_json_ld_helper_rule::check(input, &mut results);
    remap_ids(
        &mut results,
        &[
            ("TS-ASTRO-CONFIG-13", "TS-ASTRO-SEO-CONFIG-13"),
            ("TS-ASTRO-CONFIG-14", "TS-ASTRO-SEO-CONFIG-14"),
            ("TS-ASTRO-CONFIG-15", "TS-ASTRO-SEO-CONFIG-15"),
            ("TS-ASTRO-CONFIG-16", "TS-ASTRO-SEO-CONFIG-16"),
            ("TS-ASTRO-CONFIG-17", "TS-ASTRO-SEO-CONFIG-17"),
            ("TS-ASTRO-CONFIG-22", "TS-ASTRO-SEO-CONFIG-22"),
            ("TS-ASTRO-CONFIG-31", "TS-ASTRO-SEO-CONFIG-31"),
            ("TS-ASTRO-CONFIG-32", "TS-ASTRO-SEO-CONFIG-32"),
        ],
    );
    results
}

fn remap_ids(results: &mut [G3CheckResult], ids: &[(&str, &str)]) {
    for result in results {
        for (from, to) in ids {
            if result.id() == *from {
                let replacement = G3CheckResult::new(
                    (*to).to_owned(),
                    result.severity(),
                    result.title().to_owned(),
                    result.message().to_owned(),
                    result.file().map(ToOwned::to_owned),
                    result.line(),
                );
                *result = if result.inventory() {
                    replacement.into_inventory()
                } else {
                    replacement
                };
                break;
            }
        }
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
