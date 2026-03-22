use super::super::check;
use super::super::test_support::incomplete_workspace_policy_root_tree;

#[test]
fn errors_when_local_policy_root_drops_baseline() {
    let results = check(&incomplete_workspace_policy_root_tree());
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-13" && !r.inventory && r.file.as_deref() == Some("workspace/clippy.toml") && r.message.contains("thresholds")));
}
