use g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroSeoConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for contract in &input.integration_contracts {
        crate::ts_astro_config_13_nuasite_checks::check(contract, &mut results);
        crate::ts_astro_config_14_sitemap_integration::check(contract, &mut results);
        crate::ts_astro_config_15_robots_integration::check(contract, &mut results);
        crate::ts_astro_config_16_llms_txt::check(contract, &mut results);
        crate::ts_astro_config_17_seo_packages::check(contract, &mut results);
        crate::ts_astro_config_22_structured_data_check::check(contract, &mut results);
        crate::ts_astro_config_24_strict_policy_paths::check_seo(contract, &mut results);
        crate::ts_astro_config_29_policy_helper_surfaces::check_seo(contract, &mut results);
    }
    for contract in &input.missing_metadata_helper_sources {
        crate::ts_astro_config_31_metadata_helper_rule::check_missing_source(
            contract,
            &mut results,
        );
    }
    for contract in &input.missing_json_ld_helper_sources {
        crate::ts_astro_config_32_json_ld_helper_rule::check_missing_source(contract, &mut results);
    }
    for eslint_contract in &input.eslint_contracts {
        crate::ts_astro_config_31_metadata_helper_rule::check_eslint(eslint_contract, &mut results);
        crate::ts_astro_config_32_json_ld_helper_rule::check_eslint(eslint_contract, &mut results);
    }
    results
}
