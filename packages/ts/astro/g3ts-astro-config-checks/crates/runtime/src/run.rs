use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3TsAstroConfigChecksInput) -> Vec<G3CheckResult> {
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
    crate::ts_astro_config_13_nuasite_checks::check(input, &mut results);
    crate::ts_astro_config_14_sitemap_integration::check(input, &mut results);
    crate::ts_astro_config_15_robots_integration::check(input, &mut results);
    crate::ts_astro_config_16_llms_txt::check(input, &mut results);
    crate::ts_astro_config_17_seo_packages::check(input, &mut results);
    crate::ts_astro_config_18_content_adapter_rule::check(input, &mut results);
    crate::ts_astro_config_19_inline_copy_rule::check(input, &mut results);
    crate::ts_astro_config_20_mdx_lane::check(input, &mut results);
    crate::ts_astro_config_21_required_integrations::check(input, &mut results);
    crate::ts_astro_config_22_structured_data_check::check(input, &mut results);
    crate::ts_astro_config_23_strict_content_policy::check(input, &mut results);
    crate::ts_astro_config_24_strict_policy_paths::check(input, &mut results);
    crate::ts_astro_config_25_route_scope_overlap::check(input, &mut results);
    crate::ts_astro_config_26_policy_eslint_coverage::check(input, &mut results);
    crate::ts_astro_config_27_content_adapter_exists::check(input, &mut results);
    crate::ts_astro_config_28_content_adapter_astro_content::check(input, &mut results);
    crate::ts_astro_config_29_policy_helper_surfaces::check(input, &mut results);
    crate::ts_astro_config_30_mdx_component_map_rule::check(input, &mut results);
    crate::ts_astro_config_31_metadata_helper_rule::check(input, &mut results);
    crate::ts_astro_config_32_json_ld_helper_rule::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
