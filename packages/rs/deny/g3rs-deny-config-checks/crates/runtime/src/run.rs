use g3rs_deny_config_checks_types::G3RsDenyConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Run extracted deny config checks for one parsed `deny.toml`.
#[must_use]
pub fn check(input: &G3RsDenyConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::advisories::check(&input.deny_rel_path, &input.deny, &mut results);
    crate::bans::check(&input.deny_rel_path, &input.deny, &mut results);
    crate::licenses::check(&input.deny_rel_path, &input.deny, &mut results);
    crate::sources::check(&input.deny_rel_path, &input.deny, &mut results);
    crate::rs_deny_config_23_ban_baseline_complete::check(input, &mut results);
    crate::rs_deny_config_24_license_exceptions_inventory::check(input, &mut results);
    crate::rs_deny_config_25_allow_override_channel::check(input, &mut results);
    crate::rs_deny_config_26_extra_deny_bans_inventory::check(input, &mut results);
    crate::rs_deny_config_27_wrappers::check(input, &mut results);

    results
}
