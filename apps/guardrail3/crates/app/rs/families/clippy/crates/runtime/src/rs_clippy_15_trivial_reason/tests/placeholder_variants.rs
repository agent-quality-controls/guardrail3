use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn stays_quiet_for_all_placeholder_reason_variants_on_ban_entries() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = [
    { path = "method::empty", reason = "" },
    { path = "method::space", reason = "   " },
    { path = "method::fixme", reason = "fixme" },
]
disallowed-types = [
    { path = "type::later", reason = "fix later" },
    { path = "type::tbd", reason = "tbd" },
]
disallowed-macros = [
    { path = "macro::dots", reason = "..." },
]
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assert!(results.is_empty(), "ban reasons do not matter for harsher policy: {results:#?}");
}
