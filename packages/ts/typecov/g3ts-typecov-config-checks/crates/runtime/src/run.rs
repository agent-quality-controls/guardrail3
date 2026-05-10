use g3ts_typecov_types::G3TsTypecovConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Runs all typecov config checks and returns aggregated results.
#[must_use]
pub fn check(input: &G3TsTypecovConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    if input.contracts.is_empty() {
        results.push(crate::common::error(
            "g3ts-typecov/policy-configured",
            "Typecov policy root is missing",
            "`package.json` must exist so G3TS can evaluate typecov policy for this app/package root.".to_owned(),
            None,
        ));
        return results;
    }
    for contract in &input.contracts {
        if let Some(policy_result) = crate::policy_configured::check(contract) {
            results.push(policy_result);
            continue;
        }
        results.push(crate::package_present::check(contract));
        results.push(crate::script_present::check(contract));
        results.push(crate::threshold_fail_closed::check(contract));
        results.push(crate::validate_runs_typecov::check(contract));
        results.push(crate::syncpack_type_coverage_pin::check(contract));
    }
    results
}
