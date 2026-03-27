use guardrail3_app_rs_family_clippy_assertions::rs_clippy_24_forbid_clippy_conf_dir_override as assertions;
use test_support::nested_workspace_member_with_cargo_config;

use super::super::run_for_tests;

#[test]
fn errors_for_workspace_member_cargo_config_override() {
    let tree = nested_workspace_member_with_cargo_config(
        "config.toml",
        "[env]\nCLIPPY_CONF_DIR = \"../../..\"\n",
    );
    let results = run_for_tests(&tree);
    assertions::assert_override_error(
        &results,
        "apps/backend/crates/core/.cargo/config.toml",
    );
}
