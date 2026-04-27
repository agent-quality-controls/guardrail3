use g3ts_astro_types::G3TsAstroStateFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroStateFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::ts_astro_filetree_11_no_legacy_parallel_state::check(input, &mut results);
    crate::ts_astro_filetree_12_configured_forbidden_state::check(input, &mut results);
    results
}
