use super::super::check;
use super::super::test_support::root_workspace_tree;

#[test]
fn warns_on_missing_reason_entries() {
    let results = check(&root_workspace_tree(r#"disallowed-methods = ["std::env::var"]"#));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-08" && !r.inventory));
}
