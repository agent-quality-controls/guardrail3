use guardrail3_app_rs_family_clippy_assertions::rs_clippy_17_test_relaxations as assertions;
use test_support::{build_fixture_clippy_toml, root_workspace_tree};

use super::super::run_for_tests;

#[test]
fn emits_no_result_for_generated_test_relaxation_baseline() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_no_results(&results);
}
