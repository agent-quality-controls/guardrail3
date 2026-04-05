use g3_deny_content_checks_types::G3DenyContentChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run extracted deny content checks for one parsed `deny.toml`.
#[must_use]
pub fn check(input: &G3DenyContentChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::advisories::check(&input.deny_rel_path, &input.deny, &mut results);
    crate::bans::check(&input.deny_rel_path, &input.deny, &mut results);
    crate::licenses::check(&input.deny_rel_path, &input.deny, &mut results);
    crate::sources::check(&input.deny_rel_path, &input.deny, &mut results);

    results
}
