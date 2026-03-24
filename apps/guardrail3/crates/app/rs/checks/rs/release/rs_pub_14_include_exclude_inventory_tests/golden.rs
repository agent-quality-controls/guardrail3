use super::super::super::check as run_family;
use super::super::super::test_support::{
    StubToolChecker, crate_facts, crate_input, dir_entry, project_tree, temp_root,
};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn inventories_include_exclude_when_present() {
    let facts = crate_facts("x");
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-14");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].title.contains("include/exclude configured"));
    assert_eq!(
        results[0].message,
        "Cargo.toml sets `include` or `exclude`."
    );
}

#[test]
fn inventories_include_patterns_from_manifest() {
    let root = temp_root("release-include-manifest-positive");
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
include = ["src/**"]

[lib]
path = "src/lib.rs"
"#,
            ),
            ("README.md", "# Lib\n\nREADME\n"),
            ("src/lib.rs", "pub fn marker() {}\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-14"
            && result.severity == Severity::Info
            && result.inventory
            && result.file.as_deref() == Some("Cargo.toml")
    }));
}

#[test]
fn inventories_exclude_patterns_from_manifest() {
    let root = temp_root("release-exclude-manifest-positive");
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
exclude = ["tests/**"]

[lib]
path = "src/lib.rs"
"#,
            ),
            ("README.md", "# Lib\n\nREADME\n"),
            ("src/lib.rs", "pub fn marker() {}\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-14"
            && result.severity == Severity::Info
            && result.inventory
            && result.file.as_deref() == Some("Cargo.toml")
    }));
}
