use g3ts_astro_types::G3TsAstroSetupConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroSetupConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_config_01_astro_package_present::check(&input.integration_contracts, &mut results);
    crate::ts_astro_config_02_astro_check_present::check(&input.integration_contracts, &mut results);
    crate::ts_astro_config_03_astro_eslint_plugin_package_present::check(&input.integration_contracts, &mut results);
    crate::ts_astro_config_05_astro_eslint_plugin_wired::check(&input.eslint_contracts, &mut results);
    crate::ts_astro_config_06_pipeline_plugin_package_present::check(&input.integration_contracts, &mut results);
    crate::ts_astro_config_07_pipeline_plugin_wired::check(&input.eslint_contracts, &mut results);
    crate::ts_astro_config_09_syncpack_stack_pins::check(&input.integration_contracts, &mut results);
    crate::ts_astro_config_10_syncpack_forbidden_deps::check(&input.integration_contracts, &mut results);
    crate::ts_astro_config_11_site_url::check(&input.integration_contracts, &mut results);
    crate::ts_astro_config_12_static_output::check(&input.integration_contracts, &mut results);
    crate::ts_astro_config_21_required_integrations::check(&input.integration_contracts, &mut results);
    results
}
