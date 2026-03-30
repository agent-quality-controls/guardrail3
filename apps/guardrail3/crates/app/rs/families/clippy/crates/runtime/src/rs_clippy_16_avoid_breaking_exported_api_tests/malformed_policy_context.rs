use guardrail3_app_rs_family_clippy_assertions::rs_clippy_16_avoid_breaking_exported_api as assertions;
use test_support::root_workspace_tree_with_guardrail;

use super::super::run_for_tests;

#[test]
fn yields_no_result_when_policy_context_is_malformed() {
    let tree = root_workspace_tree_with_guardrail("avoid-breaking-exported-api = true", "[");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_no_result_when_policy_context_is_malformed(&results);
}

#[test]
fn yields_no_result_when_policy_context_shape_is_invalid() {
    let tree = root_workspace_tree_with_guardrail(
        "avoid-breaking-exported-api = true",
        "[profile]\nname = 7\n",
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_no_result_when_policy_context_is_malformed(&results);
}
