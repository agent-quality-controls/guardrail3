use guardrail3_app_rs_family_clippy_assertions::rs_clippy_17_test_relaxations::{
    self as assertions,
};
use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn errors_and_warns_when_managed_test_relaxation_keys_are_missing() {
    let tree = root_workspace_tree("");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_missing_messages(&results, "clippy.toml");
    assert_eq!(
        results.len(),
        5,
        "expected five managed test relaxation diagnostics"
    );
}
