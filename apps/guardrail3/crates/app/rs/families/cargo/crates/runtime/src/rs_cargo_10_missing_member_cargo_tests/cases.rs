use super::{FULL_CLIPPY_LINTS, FULL_RUST_LINTS, check_results, entry, rule_results, tree};

#[test]
fn declared_member_without_manifest_warns() {
    let workspace_manifest = format!(
        r#"
            [workspace]
            members = ["crates/api", "crates/missing"]
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
            ("crates", entry(&["api", "missing"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
            ("crates/missing", entry(&[], &[])),
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

    let rule = rule_results(&results, "RS-CARGO-10");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert_eq!(
        rule[0].title,
        "declared workspace member missing Cargo.toml"
    );
}

#[test]
fn complete_member_set_emits_no_missing_member_warning() {
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

    assert!(
        rule_results(&results, "RS-CARGO-10").is_empty(),
        "unexpected results: {results:#?}"
    );
}
