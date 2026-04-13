use g3rs_deny_config_checks_assertions::rs_deny_config_24_license_exceptions_inventory as assertions;

use crate::test_support::run;

#[test]
fn stays_quiet_when_no_license_exceptions_exist() {
    let results = run(
        "",
        Some("service"),
        true,
        crate::rs_deny_config_24_license_exceptions_inventory::check,
    );

    assertions::assert_no_findings(&results);
}
