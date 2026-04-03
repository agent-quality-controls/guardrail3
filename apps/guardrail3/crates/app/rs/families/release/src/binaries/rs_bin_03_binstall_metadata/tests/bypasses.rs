use super::helpers::check;
use super::helpers::run_tree as run_family;
use super::helpers::{StubToolChecker, dir_entry, project_tree, temp_root};
use super::helpers::{crate_facts, crate_input};
use guardrail3_app_rs_family_release_assertions::binaries::rs_bin_03_binstall_metadata as assertions;

#[test]
fn warns_without_binstall_metadata_and_skips_out_of_scope_crates() {
    let mut missing = crate_facts("bin");
    missing.is_binary = true;
    missing.has_binstall_metadata = false;
    let missing_input = crate_input(&missing);
    let mut missing_results = Vec::new();
    check(&missing_input, &mut missing_results);
    assert!(!assertions::findings(&missing_results).is_empty());
    assertions::assert_rule_results(
        &missing_results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("missing binstall metadata"),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(false),
            message: Some("Publishable binary crates should set `[package.metadata.binstall]`."),
            ..Default::default()
        }],
    );

    let library = crate_facts("lib");
    let library_input = crate_input(&library);
    let mut library_results = Vec::new();
    check(&library_input, &mut library_results);
    assert!(assertions::findings(&library_results).is_empty());
    assertions::assert_rule_quiet(&library_results);

    let mut library_with_metadata = crate_facts("lib");
    library_with_metadata.has_binstall_metadata = true;
    let library_with_metadata_input = crate_input(&library_with_metadata);
    let mut library_with_metadata_results = Vec::new();
    check(
        &library_with_metadata_input,
        &mut library_with_metadata_results,
    );
    assert!(assertions::findings(&library_with_metadata_results).is_empty());
    assertions::assert_rule_quiet(&library_with_metadata_results);

    let mut non_publishable = crate_facts("bin");
    non_publishable.is_binary = true;
    non_publishable.publishable = false;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(assertions::findings(&non_publishable_results).is_empty());
    assertions::assert_rule_quiet(&non_publishable_results);
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
[workspace]
resolver = "2"

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

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            inventory: Some(false),
            ..Default::default()
        }],
    );
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
[workspace]
resolver = "2"

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

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            file: Some("Cargo.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
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
[workspace]
resolver = "2"

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

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title_contains: Some("binstall metadata present"),
            file: Some("Cargo.toml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
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
[workspace]
resolver = "2"

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

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            file: Some("Cargo.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
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

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
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

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
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

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
}
