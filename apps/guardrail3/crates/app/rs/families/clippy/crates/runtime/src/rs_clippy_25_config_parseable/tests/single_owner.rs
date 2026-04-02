use guardrail3_app_rs_family_clippy_assertions::rs_clippy_25_config_parseable as assertions;
use test_support::root_workspace_tree;

use super::super::run_family_for_tests;

#[test]
fn malformed_clippy_config_emits_a_single_parseability_result_through_family_orchestration() {
    let tree = root_workspace_tree("not = [valid");
    let results = run_family_for_tests(&tree);
    assertions::assert_single_owner(&results, "clippy.toml");
}
