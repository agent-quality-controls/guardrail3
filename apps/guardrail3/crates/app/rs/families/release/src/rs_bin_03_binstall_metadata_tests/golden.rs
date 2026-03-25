use super::super::super::check as run_family;
use super::super::super::test_support::{
    StubToolChecker, crate_facts, crate_input, dir_entry, project_tree, temp_root,
};
use super::super::check;
use guardrail3_domain_report::Severity;

#[test]
fn inventories_binstall_metadata_for_publishable_binary_crate() {
    let mut facts = crate_facts("bin");
    facts.is_binary = true;
    facts.has_binstall_metadata = true;
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-03");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].title.contains("binstall metadata present"));
    assert_eq!(
        results[0].message,
        "`[package.metadata.binstall]` is present."
    );
}

#[test]
fn inventories_binstall_metadata_from_manifest() {
    let root = temp_root("release-binstall-manifest-positive");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["src"], &["Cargo.toml", "README.md"])),
            ("src", dir_entry(&[], &["main.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "bin"
version = "0.1.0"
edition = "2024"
description = "bin"
license = "MIT"
repository = "https://example.com/bin"

[package.metadata.binstall]
pkg-url = "{ repo }/releases/download/{ version }/{ name }"
"#,
            ),
            ("README.md", "# Bin\n\nREADME\n"),
            ("src/main.rs", "fn main() {}\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-BIN-03"
            && result.severity == Severity::Info
            && result.inventory
            && result.file.as_deref() == Some("Cargo.toml")
            && result.title.contains("binstall metadata present")
    }));
}

#[test]
fn inventories_binstall_metadata_for_autodiscovered_src_bin_manifest() {
    let root = temp_root("release-binstall-src-bin-positive");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["src"], &["Cargo.toml", "README.md"])),
            ("src", dir_entry(&["bin"], &[])),
            ("src/bin", dir_entry(&[], &["cli.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "bin"
version = "0.1.0"
edition = "2024"
description = "bin"
license = "MIT"
repository = "https://example.com/bin"

[package.metadata.binstall]
pkg-url = "{ repo }/releases/download/{ version }/{ name }"
"#,
            ),
            ("README.md", "# Bin\n\nREADME\n"),
            ("src/bin/cli.rs", "fn main() {}\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-BIN-03"
            && result.severity == Severity::Info
            && result.inventory
            && result.file.as_deref() == Some("Cargo.toml")
            && result.title.contains("binstall metadata present")
    }));
}

#[test]
fn inventories_binstall_metadata_for_explicit_bin_when_autobins_disabled() {
    let root = temp_root("release-binstall-explicit-bin-positive");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["src"], &["Cargo.toml", "README.md"])),
            ("src", dir_entry(&[], &["main.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "bin"
version = "0.1.0"
edition = "2024"
description = "bin"
license = "MIT"
repository = "https://example.com/bin"
autobins = false

[[bin]]
name = "bin"
path = "src/main.rs"

[package.metadata.binstall]
pkg-url = "{ repo }/releases/download/{ version }/{ name }"
"#,
            ),
            ("README.md", "# Bin\n\nREADME\n"),
            ("src/main.rs", "fn main() {}\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-BIN-03"
            && result.severity == Severity::Info
            && result.inventory
            && result.file.as_deref() == Some("Cargo.toml")
    }));
}
