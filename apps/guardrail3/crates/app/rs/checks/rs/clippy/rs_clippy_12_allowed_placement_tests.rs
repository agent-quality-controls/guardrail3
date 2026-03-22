use crate::domain::report::Severity;

use super::super::test_support::{collected_facts, nested_workspace_member_shadow_tree, same_root_dual_config_tree};
use super::check;

#[test]
fn rejects_nested_clippy_toml() {
    let facts = collected_facts(&nested_workspace_member_shadow_tree("clippy.toml"));
    let forbidden = facts
        .forbidden_configs
        .iter()
        .find(|forbidden| forbidden.config.rel_path == "workspace/crates/core/clippy.toml")
        .expect("expected forbidden config");
    let mut results = Vec::new();

    check(forbidden, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-12");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "clippy.toml in forbidden location");
    assert_eq!(result.file.as_deref(), Some("workspace/crates/core/clippy.toml"));
}

#[test]
fn rejects_nested_dot_clippy_toml() {
    let facts = collected_facts(&nested_workspace_member_shadow_tree(".clippy.toml"));
    let forbidden = facts
        .forbidden_configs
        .iter()
        .find(|forbidden| forbidden.config.rel_path == "workspace/crates/core/.clippy.toml")
        .expect("expected forbidden dot config");
    let mut results = Vec::new();

    check(forbidden, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-12");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "clippy.toml in forbidden location");
    assert_eq!(result.file.as_deref(), Some("workspace/crates/core/.clippy.toml"));
}

#[test]
fn rejects_same_root_dual_clippy_configs() {
    let facts = collected_facts(&same_root_dual_config_tree());
    let forbidden = facts
        .forbidden_configs
        .iter()
        .find(|forbidden| forbidden.config.rel_path == ".clippy.toml")
        .expect("expected same-root conflict");
    let mut results = Vec::new();

    check(forbidden, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-12");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "same-root clippy config conflict");
    assert_eq!(result.file.as_deref(), Some(".clippy.toml"));
    assert!(result.message.contains("clippy.toml"));
}
