use g3ts_astro_mdx_types::G3TsAstroMdxConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroMdxConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for contract in &input.integration_contracts {
        crate::ts_astro_config_24_strict_policy_paths::check_mdx(contract, &mut results);
        crate::ts_astro_config_29_policy_helper_surfaces::check_mdx(contract, &mut results);
        crate::ts_astro_config_20_mdx_lane::check_package(contract, &mut results);
    }
    for contract in &input.missing_component_map_sources {
        crate::ts_astro_config_30_mdx_component_map_rule::check_missing_source(
            contract,
            &mut results,
        );
    }
    for eslint_contract in &input.eslint_contracts {
        crate::ts_astro_config_20_mdx_lane::check_eslint(eslint_contract, &mut results);
        crate::ts_astro_config_30_mdx_component_map_rule::check_eslint(
            eslint_contract,
            &mut results,
        );
        crate::strict_component_rules::ts_astro_config_35_mdx_import_names::check_eslint(
            eslint_contract,
            &mut results,
        );
        crate::strict_component_rules::ts_astro_config_36_no_raw_ui_exports::check_eslint(
            eslint_contract,
            &mut results,
        );
        crate::strict_component_rules::ts_astro_config_37_mdx_component_wrapper_zod_parse::check_eslint(
            eslint_contract,
            &mut results,
        );
        crate::strict_component_rules::ts_astro_config_38_no_raw_mdx_images::check_eslint(
            eslint_contract,
            &mut results,
        );
    }
    results
}
