use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{entry, has_result, tree};

#[test]
fn workspace_metadata_is_inventoried() {
    let tree = tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[(
            "Cargo.toml",
            r#"
                [workspace]
                members = []
                resolver = "2"

                [workspace.package]
                edition = "2024"
                rust-version = "1.85"
            "#,
        )],
    );

    let results = check(&tree);
    assert!(has_result(&results, "RS-CARGO-05", |result| result.inventory));
}

#[test]
fn library_profile_missing_rust_version_warns() {
    let tree = tree(
        &[("", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("guardrail3.toml", "[profile]\nname = \"library\""),
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = []
                    resolver = "2"

                    [workspace.package]
                    edition = "2024"
                "#,
            ),
        ],
    );

    let results = check(&tree);
    assert!(has_result(&results, "RS-CARGO-05", |result| {
        matches!(result.severity, Severity::Warn)
    }));
}
