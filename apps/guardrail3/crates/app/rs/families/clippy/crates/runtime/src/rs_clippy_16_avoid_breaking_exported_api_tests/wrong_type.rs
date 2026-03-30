use guardrail3_app_rs_family_clippy_assertions::rs_clippy_16_avoid_breaking_exported_api as assertions;
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn warns_when_avoid_breaking_exported_api_is_not_a_bool() {
    let tree = root_workspace_tree("avoid-breaking-exported-api = \"no\"\n");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_wrong_type(&results, "clippy.toml", "string");
}
