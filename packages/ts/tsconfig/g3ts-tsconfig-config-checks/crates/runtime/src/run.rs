use g3ts_tsconfig_types::G3TsTsconfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsTsconfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::exists::check(input, &mut results);
    crate::parseable::check(input, &mut results);
    crate::extends_chain_resolves::check(input, &mut results);
    crate::extends_or_inline::check(input, &mut results);
    crate::strict_baseline::check(input, &mut results);
    results
}
