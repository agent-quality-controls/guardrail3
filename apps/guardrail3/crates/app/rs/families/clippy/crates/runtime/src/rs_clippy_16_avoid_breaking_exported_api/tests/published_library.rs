use guardrail3_app_rs_family_clippy_assertions::rs_clippy_16_avoid_breaking_exported_api as assertions;
use test_support::published_library_package_root_tree;

use super::helpers::run_for_tests;

#[test]
fn inventories_true_value_for_published_library_packages() {
    let tree = published_library_package_root_tree("avoid-breaking-exported-api = true");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_published_library(&results, "clippy.toml");
}
