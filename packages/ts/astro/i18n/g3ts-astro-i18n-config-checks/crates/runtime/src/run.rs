use g3ts_astro_i18n_types::G3TsAstroI18nConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsAstroI18nConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for contract in &input.integration_contracts {
        crate::policy_rules::check(contract, &mut results);
        crate::package_rules::check(contract, &mut results);
    }
    for contract in &input.eslint_contracts {
        crate::rule_wiring::check(contract, &mut results);
    }
    results
}
