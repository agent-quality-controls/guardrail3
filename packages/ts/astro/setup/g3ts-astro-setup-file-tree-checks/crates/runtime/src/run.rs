use g3ts_astro_setup_types::G3TsAstroSetupFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroSetupFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for app_root in &input.app_roots {
        crate::astro_config_exists::check(app_root, &mut results);
    }
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for run module.
mod run_tests;
