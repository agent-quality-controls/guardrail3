use super::helpers::run_tree_with_validation_scope;
use super::helpers::{StubToolChecker, dir_entry, project_tree, temp_root};

#[test]
fn subtree_scope_does_not_treat_publishable_sibling_path_dep_as_internal() {
    let root = temp_root("release-pub-10-validation-scope");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml"])),
            ("crates", dir_entry(&["consumer", "shared"], &[])),
            (
                "crates/consumer",
                dir_entry(&["src"], &["Cargo.toml", "README.md"]),
            ),
            ("crates/consumer/src", dir_entry(&[], &["lib.rs"])),
            (
                "crates/shared",
                dir_entry(&["src"], &["Cargo.toml", "README.md"]),
            ),
            ("crates/shared/src", dir_entry(&[], &["lib.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/*"]
resolver = "2"
"#,
            ),
            (
                "crates/consumer/Cargo.toml",
                r#"
[package]
name = "consumer"
version = "1.0.0"
edition = "2024"
description = "consumer"
license = "MIT"
repository = "https://example.com/consumer"

[dependencies]
shared = { path = "../shared", version = "1.0.0" }
"#,
            ),
            ("crates/consumer/README.md", "# Consumer\n\nrelease docs\n"),
            ("crates/consumer/src/lib.rs", "pub fn consumer() {}\n"),
            (
                "crates/shared/Cargo.toml",
                r#"
[package]
name = "shared"
version = "1.0.0"
edition = "2024"
description = "shared"
license = "MIT"
repository = "https://example.com/shared"
"#,
            ),
            ("crates/shared/README.md", "# Shared\n\nrelease docs\n"),
            ("crates/shared/src/lib.rs", "pub fn shared() {}\n"),
        ],
        root,
    );

    let results = run_tree_with_validation_scope(
        &tree,
        &StubToolChecker::new(true),
        false,
        "crates/consumer/src",
    );

    assert!(
        !results.iter().any(|result| {
            result.id() == "RS-PUB-10" && result.file() == Some("crates/consumer/Cargo.toml")
        }),
        "{results:#?}"
    );
}
