use guardrail3_app_rs_family_clippy_assertions::rs_clippy_24_forbid_clippy_conf_dir_override as assertions;
use test_support::root_workspace_tree_with_cargo_config;

use super::helpers::run_for_tests;

#[test]
fn errors_for_legacy_cargo_config_override() {
    let tree = root_workspace_tree_with_cargo_config("config", "[env]\nCLIPPY_CONF_DIR = \".\"\n");
    let results = run_for_tests(&tree);
    assertions::assert_override_error(&results, ".cargo/config");
}
