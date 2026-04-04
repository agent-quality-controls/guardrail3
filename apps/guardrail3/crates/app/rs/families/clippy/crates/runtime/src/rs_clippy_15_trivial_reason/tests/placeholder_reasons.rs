use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn stays_quiet_for_placeholder_reasons_on_ban_entries() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = [
    { path = "std::env::var", reason = "todo" },
]
disallowed-types = [
    { path = "std::collections::HashMap", reason = "reason" },
]
disallowed-macros = [
    { path = "std::println", reason = "short" },
]
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assert!(results.is_empty(), "ban reasons do not matter for harsher policy: {results:#?}");
}
