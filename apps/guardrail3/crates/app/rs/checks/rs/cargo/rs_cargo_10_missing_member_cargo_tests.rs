use super::super::check;
use super::super::test_support::{entry, has_result, tree};

#[test]
fn declared_member_without_cargo_toml_is_warned() {
    let tree = tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["api", "missing"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
            ("crates/missing", entry(&[], &[])),
        ],
        &[
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["crates/*"]
                    resolver = "2"

                    [workspace.package]
                    edition = "2024"
                "#,
            ),
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
    );

    let results = check(&tree);
    assert!(has_result(&results, "RS-CARGO-10", |result| {
        result.message.contains("crates/missing")
    }));
}
