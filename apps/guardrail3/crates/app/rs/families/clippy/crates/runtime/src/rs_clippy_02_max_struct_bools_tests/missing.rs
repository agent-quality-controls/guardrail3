use guardrail3_app_rs_family_clippy_assertions::rs_clippy_02_max_struct_bools as assertions;
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn errors_when_max_struct_bools_is_missing() {
    let tree = root_workspace_tree("");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_missing_value(&results);
}
