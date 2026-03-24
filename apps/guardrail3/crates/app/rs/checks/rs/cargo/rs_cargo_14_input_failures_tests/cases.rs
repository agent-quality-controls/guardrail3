use super::super::super::test_support::{
    FULL_CLIPPY_LINTS, FULL_RUST_LINTS, check_results, entry, rule_results, tree,
};

#[test]
fn malformed_owned_policy_root_manifest_surfaces_explicit_failure() {
    let results = check_results(&tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[("Cargo.toml", "[workspace")],
    ));

    let rule = rule_results(&results, "RS-CARGO-14");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert_eq!(rule[0].file.as_deref(), Some("Cargo.toml"));
}

#[test]
fn malformed_workspace_member_manifest_surfaces_explicit_failure() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = ["crates/api"]
            resolver = "2"

            [workspace.package]
            edition = "2024"
            rust-version = "1.85"

            {FULL_RUST_LINTS}
            {FULL_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["api"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("Cargo.toml", &workspace_manifest),
            ("crates/api/Cargo.toml", "[package"),
        ],
    ));

    let rule = rule_results(&results, "RS-CARGO-14");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert_eq!(rule[0].file.as_deref(), Some("crates/api/Cargo.toml"));
}

#[test]
fn malformed_root_local_guardrail_surfaces_explicit_failure() {
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
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("Cargo.toml", &workspace_manifest),
            ("guardrail3.toml", "[profile"),
        ],
    ));

    let rule = rule_results(&results, "RS-CARGO-14");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert_eq!(rule[0].file.as_deref(), Some("guardrail3.toml"));
}
