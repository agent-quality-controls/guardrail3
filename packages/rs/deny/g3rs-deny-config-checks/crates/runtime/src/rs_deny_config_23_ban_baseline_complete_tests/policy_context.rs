use g3rs_deny_config_checks_assertions::rs_deny_config_23_ban_baseline_complete as assertions;

use crate::test_support::{canonical_bans_toml, run};

#[test]
fn skips_profile_sensitive_baseline_when_policy_context_is_invalid() {
    let deny_toml = canonical_bans_toml("service").replace("\"actix-web\",\n", "");
    let results = run(
        &deny_toml,
        Some("service"),
        false,
        crate::rs_deny_config_23_ban_baseline_complete::check,
    );

    assertions::assert_no_findings(&results);
}

#[test]
fn skips_missing_bans_section_when_policy_context_is_invalid() {
    let results = run(
        "",
        Some("service"),
        false,
        crate::rs_deny_config_23_ban_baseline_complete::check,
    );

    assertions::assert_no_findings(&results);
}

#[test]
fn skips_missing_bans_deny_when_policy_context_is_invalid() {
    let results = run(
        "[bans]\n",
        Some("service"),
        false,
        crate::rs_deny_config_23_ban_baseline_complete::check,
    );

    assertions::assert_no_findings(&results);
}
