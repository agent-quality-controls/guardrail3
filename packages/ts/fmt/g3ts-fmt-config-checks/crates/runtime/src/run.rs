use g3ts_fmt_types::G3TsFmtConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Runs all fmt config checks and returns aggregated results.
#[must_use]
pub fn check(input: &G3TsFmtConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    if input.contracts.is_empty() {
        results.push(crate::common::error(
            "g3ts-fmt/policy-configured",
            "Formatter policy root is missing",
            "`package.json` must exist so G3TS can evaluate formatter policy for this app/package root.".to_owned(),
            None,
        ));
        return results;
    }
    for contract in &input.contracts {
        if let Some(policy_result) = crate::policy_configured::check(contract) {
            results.push(policy_result);
            continue;
        }
        results.push(crate::prettier_package_present::check(contract));
        results.push(crate::prettier_config_present::check(contract));
        results.push(crate::format_scripts::check(contract));
        results.push(crate::format_check_fail_closed::check(contract));
        results.push(crate::validate_runs_format_check::check(contract));
        results.push(crate::syncpack_prettier_pin::check(contract));
    }
    results
}
