use g3rs_deny_types::G3RsDenyFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3RsDenyFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::coverage::check(input, &mut results);
    crate::shadowing::check(input, &mut results);

    results
}
