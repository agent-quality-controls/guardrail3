use guardrail3_app_rs_family_clippy_assertions::rs_clippy_17_test_relaxations::{
    self as assertions,
};
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn errors_and_warns_when_managed_test_relaxation_keys_have_wrong_types() {
    let tree = root_workspace_tree(
        r#"
allow-dbg-in-tests = "no"
allow-print-in-tests = 1
allow-expect-in-tests = []
allow-panic-in-tests = { nope = true }
allow-unwrap-in-tests = 3.14
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_wrong_type_messages(&results, "clippy.toml");
    assert_eq!(
        results.len(),
        5,
        "expected five managed test relaxation diagnostics"
    );
}
