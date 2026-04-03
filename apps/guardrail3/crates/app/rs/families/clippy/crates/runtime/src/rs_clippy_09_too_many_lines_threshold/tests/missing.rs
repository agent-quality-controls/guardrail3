use guardrail3_app_rs_family_clippy_assertions::rs_clippy_09_too_many_lines_threshold as assertions;
use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn errors_when_too_many_lines_threshold_is_missing() {
    let tree = root_workspace_tree("");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_missing_value(&results);
}
