use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn stays_quiet_when_user_added_bans_are_stricter_than_baseline() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = [
    { path = "custom::method", reason = "Project-specific boundary must stay on the approved adapter surface." },
]
disallowed-types = [
    { path = "custom::Type", reason = "Avoid leaking crate-local type erasure into downstream boundaries." },
]
disallowed-macros = [
    { path = "custom::macro", reason = "Macro expansion here would hide policy-sensitive control flow." },
]
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assert!(results.is_empty(), "extra bans should stay quiet: {results:#?}");
}
