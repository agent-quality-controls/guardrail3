use g3ts_astro_media_types::G3TsAstroMediaConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroMediaConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for contract in &input.integration_contracts {
        crate::policy_rules::check(contract, &mut results);
        crate::package_rules::check(contract, &mut results);
        crate::media_assets_integration::check(contract, &mut results);
        crate::media_build_validation::check(contract, &mut results);
    }
    for contract in &input.eslint_contracts {
        crate::rule_wiring::check(contract, &mut results);
    }
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for run module.
mod run_tests;
