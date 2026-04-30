use g3ts_astro_state_types::G3TsAstroStateFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroStateFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for legacy_path in &input.legacy_generated_paths {
        crate::no_legacy_parallel_state::check(
            &legacy_path.app_root_rel_path,
            &legacy_path.rel_path,
            &mut results,
        );
    }
    for forbidden_path in &input.forbidden_state_paths {
        crate::configured_forbidden_state::check(
            &forbidden_path.app_root_rel_path,
            &forbidden_path.rel_path,
            &mut results,
        );
    }
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for run module.
mod run_tests;
