use g3rs_deny_config_checks_assertions::rs_deny_config_23_ban_baseline_complete as assertions;

use crate::test_support::{canonical_bans_toml, run};

#[test]
fn stays_quiet_for_canonical_service_bans() {
    let results = run(
        &canonical_bans_toml("service"),
        Some("service"),
        true,
        crate::rs_deny_config_23_ban_baseline_complete::check,
    );

    assertions::assert_no_findings(&results);
}
