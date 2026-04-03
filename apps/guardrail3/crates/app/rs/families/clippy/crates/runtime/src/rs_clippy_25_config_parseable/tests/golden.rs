use super::helpers::run_for_tests;
use guardrail3_app_rs_family_clippy_assertions::rs_clippy_25_config_parseable as assertions;

#[test]
fn inventories_when_clippy_config_parses() {
    let tree = test_support::root_workspace_tree(assertions::service_clippy_toml());
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_inventory(&results, "clippy.toml");
}
