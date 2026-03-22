use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{entry, has_result, tree};

#[test]
fn members_inheriting_workspace_lints_inventory_pass() {
    let tree = tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["api", "domain"], &[])),
            ("crates/api", entry(&[], &["Cargo.toml"])),
            ("crates/domain", entry(&[], &["Cargo.toml"])),
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
            (
                "crates/domain/Cargo.toml",
                r#"
                    [package]
                    name = "domain"
                    edition = "2024"

                    [lints]
                    workspace = true
                "#,
            ),
        ],
    );

    let results = check(&tree);
    assert!(has_result(&results, "RS-CARGO-04", |result| {
        result.inventory && result.file.as_deref() == Some("crates/api/Cargo.toml")
    }));
    assert!(has_result(&results, "RS-CARGO-04", |result| {
        result.inventory && result.file.as_deref() == Some("crates/domain/Cargo.toml")
    }));
}

#[test]
fn non_inheriting_member_errors() {
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

    let results = check(&tree);
    assert!(has_result(&results, "RS-CARGO-04", |result| {
        matches!(result.severity, Severity::Error)
    }));
}
