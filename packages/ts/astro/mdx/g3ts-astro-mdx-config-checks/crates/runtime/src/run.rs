use g3ts_astro_mdx_types::G3TsAstroMdxConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroMdxConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for contract in &input.integration_contracts {
        crate::strict_policy_paths::check_mdx(contract, &mut results);
        crate::policy_helper_surfaces::check_mdx(contract, &mut results);
        crate::mdx_lane::check_package(contract, &mut results);
    }
    for contract in &input.missing_component_map_sources {
        crate::mdx_component_map_rule::check_missing_source(contract, &mut results);
    }
    for eslint_contract in &input.eslint_contracts {
        crate::mdx_lane::check_eslint(eslint_contract, &mut results);
        crate::mdx_component_map_rule::check_eslint(eslint_contract, &mut results);
        crate::strict_component_rules::mdx_import_names::check_eslint(
            eslint_contract,
            &mut results,
        );
        crate::strict_component_rules::no_raw_ui_exports::check_eslint(
            eslint_contract,
            &mut results,
        );
        crate::strict_component_rules::mdx_component_wrapper_zod_parse::check_eslint(
            eslint_contract,
            &mut results,
        );
        crate::strict_component_rules::no_raw_mdx_images::check_eslint(
            eslint_contract,
            &mut results,
        );
        crate::eslint_suppression::disable_descriptions_required::check(
            eslint_contract,
            &mut results,
        );
        crate::eslint_suppression::unused_disables_fail::check(eslint_contract, &mut results);
        crate::eslint_suppression::protected_rule_disables_restricted::check(
            eslint_contract,
            &mut results,
        );
    }
    crate::eslint_suppression::disable_inventory::check_all(&input.eslint_directives, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for run module.
mod run_tests;
