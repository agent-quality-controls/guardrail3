use super::helpers::check;
use super::helpers::run_tree as run_family;
use super::helpers::{StubToolChecker, crate_facts, crate_input, dir_entry, project_tree, temp_root};
use guardrail3_app_rs_family_release_assertions::binaries::rs_bin_03_binstall_metadata as assertions;

#[test]
fn inventories_binstall_metadata_for_publishable_binary_crate() {
    let mut facts = crate_facts("bin");
    facts.is_binary = true;
    facts.has_binstall_metadata = true;
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title_contains: Some("binstall metadata present"),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(true),
            message: Some("`[package.metadata.binstall]` is present."),
            ..Default::default()
        }],
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
pkg-url = "{ repo }/releases/download/{ version }/{ name }"
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
            severity: Some(assertions::Severity::Info),
            title_contains: Some("binstall metadata present"),
            file: Some("Cargo.toml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
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
pkg-url = "{ repo }/releases/download/{ version }/{ name }"
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
            severity: Some(assertions::Severity::Info),
            title_contains: Some("binstall metadata present"),
            file: Some("Cargo.toml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
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
[workspace]
resolver = "2"

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

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("Cargo.toml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
