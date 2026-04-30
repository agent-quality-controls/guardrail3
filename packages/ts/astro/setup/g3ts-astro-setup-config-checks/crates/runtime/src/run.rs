use g3ts_astro_setup_types::G3TsAstroSetupConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroSetupConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for contract in &input.integration_contracts {
        crate::astro_package_present::check(contract, &mut results);
        crate::astro_check_present::check(contract, &mut results);
        crate::astro_eslint_plugin_package_present::check(contract, &mut results);
        crate::eslint_comments_plugin_package_present::check(contract, &mut results);
        crate::lint_script::check(contract, &mut results);
        crate::syncpack_lint_script::check(contract, &mut results);
        crate::validate_script::check(contract, &mut results);
        crate::forbidden_script_targets::check(contract, &mut results);
    }
    for contract in &input.eslint_contracts {
        crate::astro_eslint_plugin_wired::check(contract, &mut results);
        crate::eslint_disable_descriptions_required::check(contract, &mut results);
        crate::unused_eslint_disables_fail::check(contract, &mut results);
        crate::protected_setup_rule_disables_restricted::check(contract, &mut results);
    }
    for contract in &input.integration_contracts {
        crate::syncpack_stack_pins::check(contract, &mut results);
        crate::syncpack_forbidden_deps::check(contract, &mut results);
        crate::site_url::check(contract, &mut results);
        crate::static_output::check(contract, &mut results);
        crate::required_integrations::check(contract, &mut results);
    }
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for run module.
mod run_tests;
