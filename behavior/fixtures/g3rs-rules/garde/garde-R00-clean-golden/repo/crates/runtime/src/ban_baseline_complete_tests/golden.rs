use g3rs_deny_config_checks_assertions::ban_baseline_complete as assertions;

use test_support::run;

use super::helpers;

#[test]
fn stays_quiet_for_canonical_service_bans() {
    let results = run(
        helpers::service_canonical_bans_toml(),
        Some(g3rs_toml_parser::types::RustProfile::Service),
        true,
        crate::ban_baseline_complete::check,
    );

    assertions::assert_no_findings(&results);
}
