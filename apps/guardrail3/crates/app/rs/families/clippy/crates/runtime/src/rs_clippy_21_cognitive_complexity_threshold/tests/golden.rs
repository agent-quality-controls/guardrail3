use guardrail3_app_rs_family_clippy_assertions::rs_clippy_21_cognitive_complexity_threshold as assertions;
use test_support::{build_fixture_clippy_toml, root_workspace_tree};

use super::helpers::run_for_tests;

#[test]
fn inventories_generated_cognitive_complexity_threshold_baseline() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_golden(&results, "clippy.toml");
}
