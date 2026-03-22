use crate::domain::report::Severity;

use super::super::inputs::WorkspaceCargoInput;
use super::super::test_support::{collected_facts, entry, has_result, tree};
use super::check;

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

    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&WorkspaceCargoInput::new(&facts.workspace), &mut results);
    assert!(has_result(&results, "RS-CARGO-08", |result| {
        result.inventory
            && result.title == "workspace resolver set"
            && result.message == "Workspace resolver = `2`"
    }));
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

    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&WorkspaceCargoInput::new(&facts.workspace), &mut results);
    assert!(has_result(&results, "RS-CARGO-08", |result| {
        matches!(result.severity, Severity::Error)
            && result.title == "virtual workspace missing resolver"
            && result.message == "Virtual workspaces must set `resolver = \"2\"` or `resolver = \"3\"`."
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

    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&WorkspaceCargoInput::new(&facts.workspace), &mut results);
    assert!(has_result(&results, "RS-CARGO-08", |result| {
        matches!(result.severity, Severity::Error) && result.title.contains("pre-2021")
    }));
}

#[test]
fn modern_non_virtual_workspace_can_omit_resolver() {
    let tree = tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[(
            "Cargo.toml",
            r#"
                [package]
                name = "app"
                edition = "2024"

                [workspace]
                members = []
            "#,
        )],
    );

    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&WorkspaceCargoInput::new(&facts.workspace), &mut results);
    assert!(has_result(&results, "RS-CARGO-08", |result| {
        result.inventory
            && result.title == "resolver omitted on non-virtual workspace"
            && result.message
                == "Resolver is omitted, but this root package uses edition 2021+ so Cargo can infer a modern resolver."
    }));
}
