use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_app_rs_family_clippy_assertions::rs_clippy_13_local_policy_root_baseline as assertions;
use test_support::library_workspace_root_tree;

use super::super::run_for_tests;

#[test]
fn inventories_when_local_policy_root_keeps_full_managed_baseline() {
    let tree = library_workspace_root_tree(build_clippy_toml("library", false, true, "", ""));
    let results = run_for_tests(&tree, "apps/libsite/clippy.toml");
    assertions::assert_self_contained_inventory(&results, "apps/libsite/clippy.toml");
}
