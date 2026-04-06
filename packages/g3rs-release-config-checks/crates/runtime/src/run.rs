use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run all release config checks and return the collected results.
pub fn check(input: &G3RsReleaseConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_release_config_01_description_present::check(input, &mut results);
    crate::rs_release_config_02_license_present::check(input, &mut results);
    crate::rs_release_config_03_repository_present::check(input, &mut results);
    crate::rs_release_config_04_keywords_present::check(input, &mut results);
    crate::rs_release_config_05_categories_present::check(input, &mut results);
    crate::rs_release_config_06_valid_semver::check(input, &mut results);
    crate::rs_release_config_07_docs_rs_metadata::check(input, &mut results);
    crate::rs_release_config_08_binstall_metadata::check(input, &mut results);
    crate::rs_release_config_09_accidentally_publishable::check(input, &mut results);
    crate::rs_release_config_10_release_plz_baseline::check(input, &mut results);
    crate::rs_release_config_11_cliff_baseline::check(input, &mut results);
    results
}
