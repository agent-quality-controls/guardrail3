use test_support::{build_fixture_clippy_toml, root_workspace_tree_with_guardrail};

use super::super::run_for_tests;

#[test]
fn yields_no_result_when_policy_context_is_malformed() {
    let tree = root_workspace_tree_with_guardrail(
        build_fixture_clippy_toml("service", false, true, "", ""),
        "[profile\nname = \"service\"",
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assert!(
        results.is_empty(),
        "expected RS-CLIPPY-23 to own malformed policy context: {results:#?}"
    );
}
