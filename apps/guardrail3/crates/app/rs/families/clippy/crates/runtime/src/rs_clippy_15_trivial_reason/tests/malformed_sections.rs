use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn stays_quiet_when_ban_sections_are_malformed() {
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
    assert!(results.is_empty(), "malformed ban sections are owned elsewhere: {results:#?}");
}
