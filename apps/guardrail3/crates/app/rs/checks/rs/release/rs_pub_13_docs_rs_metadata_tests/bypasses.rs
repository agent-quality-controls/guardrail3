use super::super::super::check as run_family;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};
use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn emits_info_when_docs_rs_metadata_is_missing_and_skips_out_of_scope_crates() {
    let mut missing = crate_facts("x");
    missing.docs_rs_present = false;
    let missing_input = crate_input(&missing);
    let mut missing_results = Vec::new();
    check(&missing_input, &mut missing_results);
    assert_eq!(missing_results.len(), 1);
    assert_eq!(missing_results[0].id, "RS-PUB-13");
    assert_eq!(missing_results[0].severity, Severity::Info);
    assert!(!missing_results[0].inventory);
    assert_eq!(
        missing_results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(
        missing_results[0]
            .title
            .contains("docs.rs metadata missing")
    );
    assert!(
        missing_results[0]
            .message
            .contains("reproducible docs.rs builds")
    );

    let mut binary = crate_facts("bin");
    binary.is_library = false;
    binary.docs_rs_present = false;
    let binary_input = crate_input(&binary);
    let mut binary_results = Vec::new();
    check(&binary_input, &mut binary_results);
    assert!(binary_results.is_empty());

    let mut non_publishable = crate_facts("x");
    non_publishable.publishable = false;
    non_publishable.docs_rs_present = false;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(non_publishable_results.is_empty());

    let mut binary_with_metadata = crate_facts("bin");
    binary_with_metadata.is_library = false;
    binary_with_metadata.docs_rs_present = true;
    let binary_with_metadata_input = crate_input(&binary_with_metadata);
    let mut binary_with_metadata_results = Vec::new();

    check(
        &binary_with_metadata_input,
        &mut binary_with_metadata_results,
    );

    assert!(binary_with_metadata_results.is_empty());
}

#[test]
fn warns_when_docs_rs_table_is_empty() {
    let root = temp_root("release-docs-rs-empty-table");
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "README.md"]))],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "lib"
version = "0.1.0"
edition = "2024"
description = "lib"
license = "MIT"
repository = "https://example.com/lib"

[lib]
path = "src/lib.rs"

[package.metadata.docs.rs]
"#,
            ),
            ("README.md", "# Lib\n\nREADME\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-13"
            && result.severity == Severity::Info
            && !result.inventory
            && result.file.as_deref() == Some("Cargo.toml")
            && result.title.contains("docs.rs metadata missing")
    }));
}

#[test]
fn emits_non_inventory_info_when_docs_rs_table_has_only_unrelated_keys() {
    let root = temp_root("release-docs-rs-unrelated-key");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["src"], &["Cargo.toml", "README.md"])),
            ("src", dir_entry(&[], &["lib.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "lib"
version = "0.1.0"
edition = "2024"
description = "lib"
license = "MIT"
repository = "https://example.com/lib"

[lib]
path = "src/lib.rs"

[package.metadata.docs.rs]
foo = "bar"
"#,
            ),
            ("README.md", "# Lib\n\nREADME\n"),
            ("src/lib.rs", "pub fn marker() {}\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-13"
            && result.severity == Severity::Info
            && !result.inventory
            && result.file.as_deref() == Some("Cargo.toml")
            && result.title.contains("docs.rs metadata missing")
    }));
}
