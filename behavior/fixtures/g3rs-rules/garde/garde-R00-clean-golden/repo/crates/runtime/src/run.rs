use g3rs_deny_types::G3RsDenyConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run extracted deny config checks for one parsed `deny.toml`.
#[must_use]
pub fn check(input: &G3RsDenyConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::advisories::check(&input.deny_rel_path, &input.deny, &mut results);
    crate::bans::check(&input.deny_rel_path, &input.deny, &mut results);
    crate::licenses::check(&input.deny_rel_path, &input.deny, &mut results);
    crate::sources::check(&input.deny_rel_path, &input.deny, &mut results);
    crate::ban_baseline_complete::check(input, &mut results);
    crate::license_exceptions_inventory::check(input, &mut results);
    crate::allow_override_channel::check(input, &mut results);
    crate::extra_deny_bans_inventory::check(input, &mut results);
    crate::wrappers::check(input, &mut results);

    results
}
