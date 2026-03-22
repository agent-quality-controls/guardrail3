use crate::domain::report::Severity;

use super::super::test_support::{collected_facts, entry, member_input, tree};
use super::check;

#[test]
fn weakened_member_override_is_reported() {
    let tree = tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["api"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
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

                    [workspace.lints.rust]
                    warnings = "deny"
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

                    [lints.rust]
                    warnings = "allow"
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&member_input(&facts, "crates/api"), &mut results);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CARGO-06");
    assert!(!result.inventory);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "weakened member rust override");
    assert_eq!(
        result.message,
        "`warnings` is `allow` in the member but `deny` in the workspace."
    );
    assert_eq!(result.file.as_deref(), Some("crates/api/Cargo.toml"));
}

#[test]
fn non_inheriting_member_does_not_emit_weakened_override() {
    let tree = tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["api"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
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

                    [workspace.lints.rust]
                    warnings = "deny"
                "#,
            ),
            (
                "crates/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                    edition = "2024"

                    [lints.rust]
                    warnings = "allow"
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&member_input(&facts, "crates/api"), &mut results);
    assert!(results.is_empty());
}
