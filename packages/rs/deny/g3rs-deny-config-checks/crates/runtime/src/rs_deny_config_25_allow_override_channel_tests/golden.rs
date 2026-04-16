use g3rs_deny_config_checks_assertions::rs_deny_config_25_allow_override_channel as assertions;

use test_support::run;

use super::helpers;

#[test]
fn stays_quiet_for_canonical_baseline_without_allow_list() {
    let results = run(
        &helpers::service_canonical_bans_toml(),
        Some(guardrail3_rs_toml_parser::RustProfile::Service),
        true,
        crate::rs_deny_config_25_allow_override_channel::check,
    );

    assertions::assert_no_findings(&results);
}
