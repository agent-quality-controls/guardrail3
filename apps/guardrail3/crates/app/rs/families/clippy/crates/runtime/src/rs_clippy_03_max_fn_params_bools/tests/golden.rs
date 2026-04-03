use guardrail3_app_rs_family_clippy_assertions::rs_clippy_03_max_fn_params_bools as assertions;
use test_support::{build_fixture_clippy_toml, root_workspace_tree};

use super::helpers::run_for_tests;

#[test]
fn inventories_generated_max_fn_params_bools_baseline() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_golden(&results, "clippy.toml");
}
