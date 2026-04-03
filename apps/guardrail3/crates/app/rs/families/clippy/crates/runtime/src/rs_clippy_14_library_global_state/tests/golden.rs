use guardrail3_app_rs_family_clippy_assertions::rs_clippy_14_library_global_state as assertions;
use test_support::{
    build_fixture_clippy_toml, published_library_package_root_tree, root_workspace_tree,
};

use super::helpers::run_for_tests;

#[test]
fn emits_no_result_for_non_library_profile() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_no_results(&results);
}

#[test]
fn inventories_when_library_profile_keeps_global_state_bans() {
    let tree = published_library_package_root_tree(build_fixture_clippy_toml(
        "library", false, true, "", "",
    ));
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_inventory(&results, "clippy.toml");
}
