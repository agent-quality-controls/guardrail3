use super::super::check;
use super::super::test_support::{nested_member_shadow_tree, same_root_conflict_tree};

#[test]
fn errors_on_nested_shadowing() {
    let results = check(&nested_member_shadow_tree("deny.toml"));
    assert!(results.iter().any(|r| r.id == "RS-DENY-03" && r.file.as_deref() == Some("workspace/crates/core/deny.toml")));
}

#[test]
fn errors_on_same_root_conflict() {
    let results = check(&same_root_conflict_tree());
    assert!(results.iter().any(|r| r.id == "RS-DENY-03" && r.message.contains("multiple accepted deny configs")));
}
