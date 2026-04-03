use guardrail3_app_rs_family_clippy_assertions::rs_clippy_15_trivial_reason as assertions;
use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn inventories_when_no_documented_ban_entries_can_be_parsed() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = {}
disallowed-types = [
    123,
    { reason = "missing path" },
    { path = 9, reason = "wrong type" },
]
disallowed-macros = [
    { path = "std::println", reason = ["bad"] },
]
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_inventory(&results, "clippy.toml");
}
