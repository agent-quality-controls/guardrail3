use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroConfigChecksInput) -> Vec<G3CheckResult> {
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
    results
}
