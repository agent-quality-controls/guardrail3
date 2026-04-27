use g3ts_astro_types::G3TsAstroContentConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroContentConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_config_18_content_adapter_rule::check(input, &mut results);
    crate::ts_astro_config_19_inline_copy_rule::check(input, &mut results);
    crate::ts_astro_config_23_strict_content_policy::check_content(&input.integration_contracts, &mut results);
    crate::ts_astro_config_24_strict_policy_paths::check_content(&input.integration_contracts, &mut results);
    crate::ts_astro_config_25_route_scope_overlap::check(&input.integration_contracts, &mut results);
    crate::ts_astro_config_26_policy_eslint_coverage::check(input, &mut results);
    crate::ts_astro_config_27_content_adapter_exists::check(&input.integration_contracts, &mut results);
    crate::ts_astro_config_28_content_adapter_astro_content::check(&input.integration_contracts, &mut results);
    results
}
