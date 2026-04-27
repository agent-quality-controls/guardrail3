use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::resolver::rule as assertions;

#[test]
fn errors_when_workspace_resolver_is_missing() {
    let results = run_check("[workspace]\nmembers = []\n");
    assertions::assert_has_error(&results, "workspace resolver missing", false);
}
