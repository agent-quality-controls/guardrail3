use guardrail3_app_rs_family_clippy_assertions::rs_clippy_24_forbid_clippy_conf_dir_override as assertions;
use test_support::root_workspace_tree_with_cargo_config;

use super::super::run_for_tests;

#[test]
fn errors_when_cargo_config_env_is_not_a_table() {
    let tree = root_workspace_tree_with_cargo_config("config.toml", "env = []\n");
    let results = run_for_tests(&tree);
    assertions::assert_invalid_env_shape(&results, ".cargo/config.toml");
}
