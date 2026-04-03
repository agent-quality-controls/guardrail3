use guardrail3_app_rs_family_clippy_assertions::rs_clippy_24_forbid_clippy_conf_dir_override as assertions;
use test_support::root_workspace_tree_with_cargo_config;

use super::helpers::run_for_tests;

#[test]
fn inventories_when_no_applicable_cargo_config_override_exists() {
    let tree =
        root_workspace_tree_with_cargo_config("config.toml", "[env]\nRUSTFLAGS = \"-Dwarnings\"\n");
    let results = run_for_tests(&tree);
    assertions::assert_inventory(&results);
}
