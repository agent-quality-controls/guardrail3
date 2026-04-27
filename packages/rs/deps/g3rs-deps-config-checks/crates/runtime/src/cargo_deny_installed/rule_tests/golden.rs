use g3rs_deps_config_checks_assertions::cargo_deny_installed::rule as assertions;

use super::helpers::run_check;

#[test]
fn reports_inventory_when_cargo_deny_is_installed() {
    let results = run_check(&["cargo-deny"]);

    assertions::assert_has_info(&results, "cargo-deny installed", true);
}

#[test]
fn reports_error_when_cargo_deny_is_missing() {
    let results = run_check(&[]);

    assertions::assert_has_error(&results, "cargo-deny missing", false);
}
