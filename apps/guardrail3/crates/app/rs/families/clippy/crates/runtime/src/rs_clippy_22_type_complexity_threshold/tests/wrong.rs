use guardrail3_app_rs_family_clippy_assertions::rs_clippy_22_type_complexity_threshold as assertions;
use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn errors_when_type_complexity_threshold_is_wrong() {
    let tree = root_workspace_tree("type-complexity-threshold = 76");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_wrong_value(&results);
}
