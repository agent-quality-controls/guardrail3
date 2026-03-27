use guardrail3_app_rs_family_clippy_assertions::rs_clippy_21_cognitive_complexity_threshold as assertions;
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn errors_when_cognitive_complexity_threshold_is_wrong() {
    let tree = root_workspace_tree("cognitive-complexity-threshold = 16");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_wrong_value(&results);
}
