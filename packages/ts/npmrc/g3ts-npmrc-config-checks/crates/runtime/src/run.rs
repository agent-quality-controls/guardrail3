use g3ts_npmrc_types::G3TsNpmrcChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run every TS npmrc config rule against `input` and return the aggregated
/// `G3CheckResult` list in rule-declaration order.
#[must_use]
pub fn check(input: &G3TsNpmrcChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::root_exists::check(input, &mut results);
    crate::root_parseable::check(input, &mut results);
    crate::duplicate_keys::check(input, &mut results);
    crate::required_settings_present::check(input, &mut results);
    crate::required_settings_strong_enough::check(input, &mut results);
    crate::extra_settings_inventory::check(input, &mut results);
    results
}
