use guardrail3_app_rs_family_release_assertions::publish_integrity::rs_pub_14_include_exclude_inventory as assertions;

use super::super::check;
use super::super::run_tree as run_family;
use super::super::{StubToolChecker, crate_facts, crate_input, dir_entry, project_tree, temp_root};

#[test]
fn inventories_include_exclude_when_present() {
    let facts = crate_facts("x");
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(true),
            title_contains: Some("include/exclude configured"),
            message: Some("Cargo.toml sets `include` or `exclude`."),
            ..Default::default()
        }],
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
[workspace]
resolver = "2"

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
[workspace]
resolver = "2"

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
