use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{entry, has_result, tree};

#[test]
fn older_member_edition_is_warned() {
    let tree = tree(
        &[
            ("", entry(&["crates"], &["Cargo.toml"])),
            ("crates", entry(&["legacy"], &[])),
            ("crates/legacy", entry(&[], &["Cargo.toml"])),
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
    );

    let results = check(&tree);
    assert!(has_result(&results, "RS-CARGO-09", |result| {
        matches!(result.severity, Severity::Warn)
    }));
}
