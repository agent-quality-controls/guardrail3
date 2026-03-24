use super::super::super::test_support::{
    FULL_CLIPPY_LINTS, FULL_RUST_LINTS, check_results, entry, rule_results, tree,
};

#[test]
fn older_member_edition_warns() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = ["crates/legacy"]
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
            ("crates", entry(&["legacy"], &[])),
            ("crates/legacy", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("Cargo.toml", &workspace_manifest),
            (
                "crates/legacy/Cargo.toml",
                r#"
                    [package]
                    name = "legacy"
                    edition = "2021"

                    [lints]
                    workspace = true
                "#,
            ),
        ],
    ));

    let rule = rule_results(&results, "RS-CARGO-09");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert_eq!(rule[0].title, "member edition older than workspace");
}

#[test]
fn matching_member_edition_inventories() {
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
            (
                "crates/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                    edition = "2024"

                    [lints]
                    workspace = true
                "#,
            ),
        ],
    ));

    let rule = rule_results(&results, "RS-CARGO-09");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert!(rule[0].inventory);
}
