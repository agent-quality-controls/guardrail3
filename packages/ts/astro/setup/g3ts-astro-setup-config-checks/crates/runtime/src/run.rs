use g3ts_astro_setup_types::G3TsAstroSetupConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroSetupConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for contract in &input.integration_contracts {
        crate::ts_astro_config_01_astro_package_present::check(contract, &mut results);
        crate::ts_astro_config_02_astro_check_present::check(contract, &mut results);
        crate::ts_astro_config_03_astro_eslint_plugin_package_present::check(
            contract,
            &mut results,
        );
    }
    for contract in &input.eslint_contracts {
        crate::ts_astro_config_05_astro_eslint_plugin_wired::check(contract, &mut results);
    }
    for contract in &input.integration_contracts {
        crate::ts_astro_config_09_syncpack_stack_pins::check(contract, &mut results);
        crate::ts_astro_config_10_syncpack_forbidden_deps::check(contract, &mut results);
        crate::ts_astro_config_11_site_url::check(contract, &mut results);
        crate::ts_astro_config_12_static_output::check(contract, &mut results);
        crate::ts_astro_config_21_required_integrations::check(contract, &mut results);
    }
    results
}
