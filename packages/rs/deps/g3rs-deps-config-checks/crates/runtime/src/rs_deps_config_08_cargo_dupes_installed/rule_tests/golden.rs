use g3rs_deps_config_checks_assertions::rs_deps_config_08_cargo_dupes_installed::rule as assertions;

use super::helpers::run_check;

#[test]
fn reports_inventory_when_cargo_dupes_is_installed() {
    let results = run_check(&["cargo-dupes"]);

    assertions::assert_has_info(&results, "cargo-dupes installed", true);
}

#[test]
fn reports_warning_when_cargo_dupes_is_missing() {
    let results = run_check(&[]);

    assertions::assert_has_warn(&results, "cargo-dupes missing", false);
}
