use guardrail3_app_rs_family_clippy_assertions::rs_clippy_02_max_struct_bools as assertions;
use test_support::{canonical_clippy_toml, root_workspace_tree};

use super::super::run_for_tests;

#[test]
fn inventories_generated_max_struct_bools_baseline() {
    let tree = root_workspace_tree(canonical_clippy_toml());
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_golden(&results, "clippy.toml");
}
