use test_support::root_workspace_tree;

use super::super::run_for_tests;
use guardrail3_app_rs_family_clippy_assertions::rs_clippy_06_extra_method_ban as assertions;

#[test]
fn reports_malformed_method_sections_instead_of_clean_inventory() {
    let tree = root_workspace_tree("disallowed-methods = {}\n");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_malformed_section(&results, "clippy.toml");
}
