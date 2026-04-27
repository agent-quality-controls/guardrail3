use g3rs_deps_config_checks_assertions::gitleaks_installed::rule as assertions;

use super::helpers::run_check;

#[test]
fn reports_inventory_when_gitleaks_is_installed() {
    let results = run_check(&["gitleaks"]);

    assertions::assert_has_info(&results, "gitleaks installed", true);
}

#[test]
fn reports_error_when_gitleaks_is_missing() {
    let results = run_check(&[]);

    assertions::assert_has_error(&results, "gitleaks missing", false);
}
