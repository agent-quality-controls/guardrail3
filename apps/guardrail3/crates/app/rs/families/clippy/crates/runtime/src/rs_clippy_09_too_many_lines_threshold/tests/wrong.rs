use guardrail3_app_rs_family_clippy_assertions::rs_clippy_09_too_many_lines_threshold as assertions;
use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn errors_when_too_many_lines_threshold_is_wrong() {
    let tree = root_workspace_tree("too-many-lines-threshold = 76");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_wrong_value(&results);
}
