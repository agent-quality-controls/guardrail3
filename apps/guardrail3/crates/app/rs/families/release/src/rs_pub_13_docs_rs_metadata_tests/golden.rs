use guardrail3_app_rs_family_release_assertions::rs_pub_13_docs_rs_metadata as assertions;
use super::super::run_tree as run_family;
use super::super::{
    StubToolChecker, crate_facts, crate_input, dir_entry, project_tree, temp_root,
};
use super::super::check;

#[test]
fn inventories_docs_rs_metadata_for_publishable_library_crate() {
    let facts = crate_facts("x");
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title_contains: Some("docs.rs metadata present"),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(true),
            message: Some("`[package.metadata.docs.rs]` is present."),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_nonempty_docs_rs_table_from_manifest() {
    let root = temp_root("release-docs-rs-manifest-positive");
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
all-features = true
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
