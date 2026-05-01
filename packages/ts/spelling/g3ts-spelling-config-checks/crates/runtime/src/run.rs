use g3ts_spelling_types::G3TsSpellingConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsSpellingConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    if input.contracts.is_empty() {
        results.push(crate::common::error(
            "g3ts-spelling/policy-configured",
            "Spelling policy root is missing",
            "`package.json` must exist so G3TS can evaluate spelling policy for this app/package root.".to_owned(),
            None,
        ));
        return results;
    }
    for contract in &input.contracts {
        if let Some(policy_result) = crate::policy_configured::check(contract) {
            results.push(policy_result);
            continue;
        }
        results.push(crate::cspell_package_present::check(contract));
        results.push(crate::cspell_config_present::check(contract));
        results.push(crate::spellcheck_script::check(contract));
        results.push(crate::spellcheck_fail_closed::check(contract));
        results.push(crate::validate_runs_spellcheck::check(contract));
        results.push(crate::syncpack_cspell_pin::check(contract));
    }
    results
}
