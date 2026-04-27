use g3ts_astro_types::G3TsAstroConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_config_24_strict_policy_paths::check_mdx(input, &mut results);
    crate::ts_astro_config_29_policy_helper_surfaces::check_mdx(input, &mut results);
    crate::ts_astro_config_20_mdx_lane::check(input, &mut results);
    crate::ts_astro_config_30_mdx_component_map_rule::check(input, &mut results);
    results
}
