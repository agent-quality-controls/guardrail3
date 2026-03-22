use super::super::check;
use super::super::test_support::nested_member_shadow_tree;

#[test]
fn errors_on_forbidden_deny_location() {
    let results = check(&nested_member_shadow_tree("deny.toml"));
    assert!(results.iter().any(|r| r.id == "RS-DENY-02" && r.file.as_deref() == Some("workspace/crates/core/deny.toml")));
}
