use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::resolver::rule as assertions;

#[test]
fn inventories_when_workspace_resolver_is_supported() {
    let results = run_check("[workspace]\nmembers = []\nresolver = \"3\"\n");
    assertions::assert_has_info(&results, "workspace resolver set", true);
}
