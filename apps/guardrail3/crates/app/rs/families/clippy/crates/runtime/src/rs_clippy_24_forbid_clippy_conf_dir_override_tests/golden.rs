use guardrail3_app_rs_family_clippy_assertions::rs_clippy_24_forbid_clippy_conf_dir_override as assertions;
use test_support::root_workspace_tree_with_cargo_config;

use super::super::run_for_tests;

#[test]
fn ignores_unrelated_cargo_config_env_settings() {
    let tree =
        root_workspace_tree_with_cargo_config("config.toml", "[env]\nRUSTFLAGS = \"-Dwarnings\"\n");
    let results = run_for_tests(&tree);
    assertions::assert_no_results(&results);
}
