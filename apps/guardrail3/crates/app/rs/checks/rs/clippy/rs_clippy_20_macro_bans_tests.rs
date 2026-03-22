use super::super::check;
use super::super::test_support::root_workspace_tree;

#[test]
fn errors_when_required_macro_bans_are_missing() {
    let results = check(&root_workspace_tree(
        r#"disallowed-macros = [{ path = "println", reason = "good enough reason text" }]"#,
    ));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-20" && !r.inventory && r.message.contains("eprintln")));
}
