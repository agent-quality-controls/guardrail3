use guardrail3_app_rs_family_clippy_assertions::rs_clippy_24_forbid_clippy_conf_dir_override as assertions;
use test_support::unrelated_nested_cargo_config_tree;

use super::super::run_for_tests;

#[test]
fn ignores_nested_cargo_config_outside_routed_rust_scope() {
    let tree =
        unrelated_nested_cargo_config_tree("config.toml", "[env]\nCLIPPY_CONF_DIR = \".\"\n");
    let results = run_for_tests(&tree);
    assertions::assert_no_results(&results);
}
