use super::super::check;
use super::super::test_support::{canonical_clippy_toml, garde_disabled_root_tree, root_workspace_tree};

#[test]
fn inventories_baseline_type_bans() {
    let results = check(&root_workspace_tree(canonical_clippy_toml()));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-05" && r.inventory && r.message.contains("std::collections::HashMap")));
}

#[test]
fn skips_garde_type_bans_when_garde_disabled() {
    let mut clippy = canonical_clippy_toml().to_owned();
    for path in [
        "axum::extract::Json",
        "axum::Json",
        "axum::extract::Query",
        "axum::extract::Form",
    ] {
        clippy = clippy.replace(
            &format!("    {{ path = \"{path}\", reason = \"good enough reason text\" }},\n"),
            "",
        );
    }
    let results = check(&garde_disabled_root_tree(clippy));
    assert!(!results.iter().any(|r| r.id == "RS-CLIPPY-05" && r.message.contains("axum::extract::Json")));
}
