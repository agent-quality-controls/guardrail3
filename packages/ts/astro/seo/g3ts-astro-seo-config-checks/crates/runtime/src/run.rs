use g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroSeoConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for contract in &input.integration_contracts {
        crate::nuasite_checks::check(contract, &mut results);
        crate::sitemap_integration::check(contract, &mut results);
        crate::robots_integration::check(contract, &mut results);
        crate::llms_txt::check(contract, &mut results);
        crate::seo_packages::check(contract, &mut results);
        crate::structured_data_check::check(contract, &mut results);
        crate::strict_policy_paths::check_seo(contract, &mut results);
        crate::policy_helper_surfaces::check_seo(contract, &mut results);
    }
    for contract in &input.missing_metadata_helper_sources {
        crate::metadata_helper_rule::check_missing_source(contract, &mut results);
    }
    for contract in &input.missing_json_ld_helper_sources {
        crate::json_ld_helper_rule::check_missing_source(contract, &mut results);
    }
    for eslint_contract in &input.eslint_contracts {
        crate::metadata_helper_rule::check_eslint(eslint_contract, &mut results);
        crate::json_ld_helper_rule::check_eslint(eslint_contract, &mut results);
    }
    results
}
