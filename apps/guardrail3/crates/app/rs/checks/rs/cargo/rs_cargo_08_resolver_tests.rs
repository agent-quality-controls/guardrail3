use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{entry, has_result, tree};

#[test]
fn explicit_modern_resolver_is_inventoried() {
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
            "#,
        )],
    );

    let results = check(&tree);
    assert!(has_result(&results, "RS-CARGO-08", |result| result.inventory));
}

#[test]
fn virtual_workspace_missing_resolver_is_reported() {
    let tree = tree(
        &[("", entry(&["crates"], &["Cargo.toml"]))],
        &[(
            "Cargo.toml",
            r#"
                [workspace]
                members = ["crates/*"]

                [workspace.package]
                edition = "2024"
            "#,
        )],
    );

    let results = check(&tree);
    assert!(has_result(&results, "RS-CARGO-08", |result| {
        matches!(result.severity, Severity::Error)
    }));
}

#[test]
fn pre_2021_non_virtual_workspace_missing_resolver_is_error() {
    let tree = tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[(
            "Cargo.toml",
            r#"
                [package]
                name = "app"
                edition = "2018"

                [workspace]
                members = []
            "#,
        )],
    );

    let results = check(&tree);
    assert!(has_result(&results, "RS-CARGO-08", |result| {
        matches!(result.severity, Severity::Error) && result.title.contains("pre-2021")
    }));
}
