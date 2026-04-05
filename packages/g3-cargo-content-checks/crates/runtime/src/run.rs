use g3_cargo_content_checks_types::G3CargoContentChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
#[allow(
    clippy::missing_const_for_fn,
    reason = "this scaffold intentionally returns an empty result set until rules are extracted"
)]
pub fn check(input: &G3CargoContentChecksInput) -> Vec<G3CheckResult> {
    let _ = input;
    Vec::new()
}
