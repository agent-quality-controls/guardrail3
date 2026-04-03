use guardrail3_app_rs_family_clippy_assertions::rs_clippy_13_local_policy_root_baseline as assertions;
use test_support::library_workspace_root_tree;

use super::helpers::run_for_tests;

#[test]
fn errors_when_local_policy_root_cannot_be_parsed() {
    let tree = library_workspace_root_tree("not = [valid");
    let results = run_for_tests(&tree, "apps/libsite/clippy.toml");
    assertions::assert_parse_error(&results, "apps/libsite/clippy.toml");
}
