use super::{
    FULL_CLIPPY_LINTS, FULL_RUST_LINTS, FULL_STANDALONE_CLIPPY_LINTS, FULL_STANDALONE_RUST_LINTS,
    check_results, entry, rule_results, tree,
};

#[test]
fn explicit_workspace_resolver_is_inventory() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {FULL_RUST_LINTS}
            {FULL_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[("Cargo.toml", &workspace_manifest)],
    ));

    let rule = rule_results(&results, "RS-CARGO-08");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert!(rule[0].inventory);
}

#[test]
fn missing_workspace_resolver_is_error() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = []

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {FULL_RUST_LINTS}
            {FULL_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[("Cargo.toml", &workspace_manifest)],
    ));

    let rule = rule_results(&results, "RS-CARGO-08");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert_eq!(rule[0].title, "workspace resolver missing");
}

#[test]
fn standalone_root_does_not_emit_workspace_only_rule() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {FULL_RUST_LINTS}
            {FULL_CLIPPY_LINTS}
        "#
    );
    let standalone_manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"
            rust-version = "1.85"

            {FULL_STANDALONE_RUST_LINTS}
            {FULL_STANDALONE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &["Cargo.toml"])),
            ("tools", entry(&["helper"], &[])),
            ("tools/helper", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("Cargo.toml", &workspace_manifest),
            ("tools/helper/Cargo.toml", &standalone_manifest),
        ],
    ));

    let rule = rule_results(&results, "RS-CARGO-08");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert_eq!(rule[0].file.as_deref(), Some("Cargo.toml"));
}
