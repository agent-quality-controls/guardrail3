use guardrail3_app_rs_family_clippy_assertions::rs_clippy_12_allowed_placement as assertions;
use test_support::same_root_dual_config_tree;

use super::super::run_for_tests;

#[test]
fn rejects_lower_precedence_same_root_sibling_config() {
    let results = run_for_tests(&same_root_dual_config_tree());
    assertions::assert_same_root_conflict(&results, "clippy.toml", ".clippy.toml");
}
