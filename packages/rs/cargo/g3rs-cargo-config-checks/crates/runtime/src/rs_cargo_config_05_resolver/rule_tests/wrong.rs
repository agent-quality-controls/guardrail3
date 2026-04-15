use g3rs_cargo_config_checks_assertions::rs_cargo_config_05_resolver::rule as assertions;
use super::helpers::run_check;

#[test]
fn errors_when_workspace_resolver_is_unsupported() {
    let results = run_check("[workspace]\nmembers = []\nresolver = \"1\"\n");
    assertions::assert_has_error(&results, "unsupported workspace resolver", false);
}
