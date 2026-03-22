use super::super::check;
use super::super::test_support::{nested_workspace_member_shadow_tree, same_root_dual_config_tree};

#[test]
fn rejects_nested_clippy_toml() {
    let results = check(&nested_workspace_member_shadow_tree("clippy.toml"));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-12" && !r.inventory && r.file.as_deref() == Some("workspace/crates/core/clippy.toml")));
}

#[test]
fn rejects_nested_dot_clippy_toml() {
    let results = check(&nested_workspace_member_shadow_tree(".clippy.toml"));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-12" && !r.inventory && r.file.as_deref() == Some("workspace/crates/core/.clippy.toml")));
}

#[test]
fn rejects_same_root_dual_clippy_configs() {
    let results = check(&same_root_dual_config_tree());
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-12"
            && result.title == "same-root clippy config conflict"
            && result.file.as_deref() == Some(".clippy.toml")
    }));
}
