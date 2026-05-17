use g3ts_jscpd_types::G3TsJscpdChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run every TS jscpd config rule against `input` and return the aggregated
/// `G3CheckResult` list in rule-declaration order.
#[must_use]
pub fn check(input: &G3TsJscpdChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::root_exists::check(input, &mut results);
    crate::root_parseable::check(input, &mut results);
    crate::threshold_zero::check(input, &mut results);
    crate::absolute_true::check(input, &mut results);
    crate::required_ignores::check(input, &mut results);
    crate::format_and_inventory::check(input, &mut results);
    results
}
