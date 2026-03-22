use crate::domain::report::Severity;

use super::super::inputs::WorkspaceCargoInput;
use super::super::test_support::{collected_facts, entry, has_result, tree};
use super::check;

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

    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&WorkspaceCargoInput::new(&facts.workspace), &mut results);
    assert!(has_result(&results, "RS-CARGO-05", |result| {
        result.inventory
            && result.title == "workspace metadata"
            && result.message == "Workspace metadata: edition = 2024, rust-version = 1.85"
    }));
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

    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&WorkspaceCargoInput::new(&facts.workspace), &mut results);
    assert!(has_result(&results, "RS-CARGO-05", |result| {
        matches!(result.severity, Severity::Warn)
            && result.title == "library workspace rust-version missing"
            && result.message == "Library profile should declare `rust-version` as an MSRV contract."
    }));
}

#[test]
fn outdated_workspace_edition_warns_explicitly() {
    let tree = tree(
        &[("", entry(&[], &["Cargo.toml"]))],
        &[(
            "Cargo.toml",
            r#"
                [workspace]
                members = []
                resolver = "2"

                [workspace.package]
                edition = "2018"
            "#,
        )],
    );

    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&WorkspaceCargoInput::new(&facts.workspace), &mut results);
    assert!(has_result(&results, "RS-CARGO-05", |result| {
        matches!(result.severity, Severity::Warn)
            && result.title == "outdated workspace edition"
            && result.message == "Workspace edition is `2018`. Use edition `2024` or `2021` minimum."
    }));
}
