use g3ts_astro_content_types::G3TsAstroContentConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroContentConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for contract in &input.integration_contracts {
        crate::ts_astro_config_17_pipeline_plugin_package_present::check(contract, &mut results);
        crate::ts_astro_config_19_inline_copy_rule::check_package(contract, &mut results);
    }
    for contract in &input.policy_eslint_contracts {
        crate::ts_astro_config_18_content_adapter_rule::check(contract, &mut results);
    }
    for eslint_contract in &input.eslint_contracts {
        crate::ts_astro_config_19_inline_copy_rule::check_eslint(eslint_contract, &mut results);
    }
    for contract in &input.integration_contracts {
        crate::ts_astro_config_23_strict_content_policy::check_content(contract, &mut results);
        crate::ts_astro_config_24_strict_policy_paths::check_content(contract, &mut results);
        crate::ts_astro_config_25_route_scope_overlap::check(contract, &mut results);
    }
    for contract in &input.policy_eslint_contracts {
        crate::ts_astro_config_26_policy_eslint_coverage::check(contract, &mut results);
    }
    for contract in &input.adapter_root_contracts {
        crate::ts_astro_config_27_content_adapter_exists::check(contract, &mut results);
    }
    for contract in &input.adapter_source_contracts {
        crate::ts_astro_config_28_content_adapter_astro_content::check(contract, &mut results);
    }
    results
}
