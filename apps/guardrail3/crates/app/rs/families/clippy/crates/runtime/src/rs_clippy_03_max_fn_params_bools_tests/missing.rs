use guardrail3_app_rs_family_clippy_assertions::rs_clippy_03_max_fn_params_bools as assertions;
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn errors_when_max_fn_params_bools_is_missing() {
    let tree = root_workspace_tree("");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_missing_value(&results);
}
