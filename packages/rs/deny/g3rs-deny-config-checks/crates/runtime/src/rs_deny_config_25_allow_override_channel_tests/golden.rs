use g3rs_deny_config_checks_assertions::rs_deny_config_25_allow_override_channel as assertions;

use crate::test_support::{canonical_bans_toml, run};

#[test]
fn stays_quiet_for_canonical_baseline_without_allow_list() {
    let results = run(
        &canonical_bans_toml("service"),
        Some("service"),
        true,
        crate::rs_deny_config_25_allow_override_channel::check,
    );

    assertions::assert_no_findings(&results);
}
