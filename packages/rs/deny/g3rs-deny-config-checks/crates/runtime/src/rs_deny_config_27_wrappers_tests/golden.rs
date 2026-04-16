use g3rs_deny_config_checks_assertions::rs_deny_config_27_wrappers as assertions;

use test_support::run;

use super::helpers;

#[test]
fn stays_quiet_for_canonical_wrapper_policy() {
    let results = run(
        &helpers::service_canonical_bans_toml(),
        Some(guardrail3_rs_toml_parser::RustProfile::Service),
        true,
        crate::rs_deny_config_27_wrappers::check,
    );

    assertions::assert_no_findings(&results);
}
