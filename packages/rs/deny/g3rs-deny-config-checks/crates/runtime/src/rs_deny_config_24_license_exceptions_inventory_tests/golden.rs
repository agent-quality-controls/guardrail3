use g3rs_deny_config_checks_assertions::rs_deny_config_24_license_exceptions_inventory as assertions;

use test_support::run;
use guardrail3_rs_toml_parser::RustProfile;

#[test]
fn stays_quiet_when_no_license_exceptions_exist() {
    let results = run(
        "",
        Some(RustProfile::Service),
        true,
        crate::rs_deny_config_24_license_exceptions_inventory::check,
    );

    assertions::assert_no_findings(&results);
}
