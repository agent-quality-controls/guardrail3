use guardrail3_app_rs_family_clippy_assertions::rs_clippy_02_max_struct_bools as assertions;
use test_support::{canonical_clippy_toml, library_workspace_root_tree};

use super::super::run_for_tests;

#[test]
fn inventories_generated_threshold_at_a_local_policy_root_too() {
    let tree = library_workspace_root_tree(canonical_clippy_toml());
    let results = run_for_tests(&tree, "apps/libsite/clippy.toml");
    assertions::assert_golden(&results, "apps/libsite/clippy.toml");
}
