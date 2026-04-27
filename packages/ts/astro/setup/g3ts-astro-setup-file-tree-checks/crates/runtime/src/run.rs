use g3ts_astro_types::G3TsAstroFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_filetree_01_astro_config_exists::check(input, &mut results);
    crate::ts_astro_filetree_03_live_config_exists::check(input, &mut results);
    results
}
