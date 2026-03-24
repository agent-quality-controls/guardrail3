use super::super::super::check as run_family;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};
use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn warns_without_binstall_metadata_and_skips_out_of_scope_crates() {
    let mut missing = crate_facts("bin");
    missing.is_binary = true;
    missing.has_binstall_metadata = false;
    let missing_input = crate_input(&missing);
    let mut missing_results = Vec::new();
    check(&missing_input, &mut missing_results);
    assert_eq!(missing_results.len(), 1);
    assert_eq!(missing_results[0].id, "RS-BIN-03");
    assert_eq!(missing_results[0].severity, Severity::Warn);
    assert!(!missing_results[0].inventory);
    assert_eq!(
        missing_results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(
        missing_results[0]
            .title
            .contains("missing binstall metadata")
    );
    assert_eq!(
        missing_results[0].message,
        "Publishable binary crates should set `[package.metadata.binstall]`."
    );

    let library = crate_facts("lib");
    let library_input = crate_input(&library);
    let mut library_results = Vec::new();
    check(&library_input, &mut library_results);
    assert!(library_results.is_empty());

    let mut library_with_metadata = crate_facts("lib");
    library_with_metadata.has_binstall_metadata = true;
    let library_with_metadata_input = crate_input(&library_with_metadata);
    let mut library_with_metadata_results = Vec::new();
    check(
        &library_with_metadata_input,
        &mut library_with_metadata_results,
    );
    assert!(library_with_metadata_results.is_empty());

    let mut non_publishable = crate_facts("bin");
    non_publishable.is_binary = true;
    non_publishable.publishable = false;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(non_publishable_results.is_empty());
}

#[test]
fn warns_when_binstall_metadata_is_the_wrong_shape() {
    let root = temp_root("release-binstall-wrong-shape");
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

[package.metadata]
binstall = "oops"
"#,
            ),
            ("src/main.rs", "fn main() {}\n"),
            ("README.md", "# Bin\n\nREADME\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-BIN-03" && result.severity == Severity::Warn && !result.inventory
    }));
}

#[test]
fn warns_for_autodiscovered_src_bin_manifest_without_binstall_metadata() {
    let root = temp_root("release-binstall-src-bin-missing");
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
            && result.severity == Severity::Warn
            && !result.inventory
            && result.file.as_deref() == Some("Cargo.toml")
    }));
}

#[test]
fn inventories_when_binstall_table_is_empty() {
    let root = temp_root("release-binstall-empty-table");
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
"#,
            ),
            ("src/main.rs", "fn main() {}\n"),
            ("README.md", "# Bin\n\nREADME\n"),
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
fn warns_for_explicit_bin_manifest_without_binstall_metadata() {
    let root = temp_root("release-binstall-explicit-bin-missing");
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
name = "my-package"
version = "0.1.0"
edition = "2024"
description = "bin"
license = "MIT"
repository = "https://example.com/bin"
autobins = false

[[bin]]
name = "cli"
path = "src/main.rs"
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
            && result.severity == Severity::Warn
            && !result.inventory
            && result.file.as_deref() == Some("Cargo.toml")
    }));
}

#[test]
fn does_not_emit_for_autobins_disabled_package_with_src_main() {
    let root = temp_root("release-autobins-disabled-bin-03");
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
name = "not-a-bin"
version = "0.1.0"
edition = "2024"
description = "not-a-bin"
license = "MIT"
repository = "https://example.com/not-a-bin"
autobins = false

[package.metadata.binstall]
pkg-url = "{ repo }/releases/download/{ version }/{ name }"
"#,
            ),
            ("src/main.rs", "fn main() {}\n"),
            ("README.md", "# Not A Bin\n\nREADME\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(
        results.iter().all(|result| result.id != "RS-BIN-03"),
        "autobins=false package should stay out of RS-BIN-03: {results:#?}"
    );
}

#[test]
fn does_not_emit_for_non_binary_manifest_with_binstall_metadata() {
    let root = temp_root("release-binstall-non-binary-with-metadata");
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

[package.metadata.binstall]
pkg-url = "{ repo }/releases/download/{ version }/{ name }"
"#,
            ),
            ("README.md", "# Lib\n\nREADME\n"),
            ("src/lib.rs", "pub fn value() -> u8 { 1 }\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(
        results.iter().all(|result| result.id != "RS-BIN-03"),
        "non-binary crate should stay out of RS-BIN-03: {results:#?}"
    );
}

#[test]
fn does_not_emit_for_autobins_disabled_package_with_src_bin() {
    let root = temp_root("release-autobins-disabled-src-bin-03");
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
name = "not-a-bin"
version = "0.1.0"
edition = "2024"
description = "not-a-bin"
license = "MIT"
repository = "https://example.com/not-a-bin"
autobins = false

[package.metadata.binstall]
pkg-url = "{ repo }/releases/download/{ version }/{ name }"
"#,
            ),
            ("README.md", "# Not A Bin\n\nREADME\n"),
            ("src/bin/cli.rs", "fn main() {}\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(
        results.iter().all(|result| result.id != "RS-BIN-03"),
        "autobins=false src/bin package should stay out of RS-BIN-03: {results:#?}"
    );
}
