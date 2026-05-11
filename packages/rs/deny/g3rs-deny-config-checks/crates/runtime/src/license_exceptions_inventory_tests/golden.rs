use g3rs_deny_config_checks_assertions::license_exceptions_inventory as assertions;

use g3rs_toml_parser::types::RustProfile;
use test_support::run;

#[test]
fn stays_quiet_when_no_license_exceptions_exist() {
    let results = run(
        "",
        Some(RustProfile::Service),
        true,
        crate::license_exceptions_inventory::check,
    );

    assertions::assert_no_findings(&results);
}
