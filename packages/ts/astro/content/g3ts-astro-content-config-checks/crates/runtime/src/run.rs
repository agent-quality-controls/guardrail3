use g3ts_astro_content_types::G3TsAstroContentConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroContentConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for contract in &input.integration_contracts {
        crate::pipeline_plugin_package_present::check(contract, &mut results);
        crate::inline_copy_rule::check_package(contract, &mut results);
    }
    for contract in &input.policy_eslint_contracts {
        crate::content_adapter_rule::check(contract, &mut results);
    }
    for eslint_contract in &input.eslint_contracts {
        crate::inline_copy_rule::check_eslint(eslint_contract, &mut results);
        crate::protected_rule_disables_restricted::check(eslint_contract, &mut results);
    }
    for contract in &input.integration_contracts {
        crate::strict_content_policy::check_content(contract, &mut results);
        crate::strict_policy_paths::check_content(contract, &mut results);
        crate::route_scope_overlap::check(contract, &mut results);
    }
    for contract in &input.policy_eslint_contracts {
        crate::policy_eslint_coverage::check(contract, &mut results);
    }
    for contract in &input.adapter_root_contracts {
        crate::content_adapter_exists::check(contract, &mut results);
    }
    for contract in &input.adapter_source_contracts {
        crate::content_adapter_astro_content::check(contract, &mut results);
    }
    crate::eslint_disable_inventory::check_all(&input.eslint_directives, &mut results);
    results
}
