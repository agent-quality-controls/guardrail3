use super::helpers::run_tree_with_validation_scope as run_family;
use super::helpers::{StubToolChecker, dir_entry, project_tree, temp_root};
use guardrail3_app_rs_family_release_assertions::publish_metadata::rs_pub_01_description_present as assertions;

#[test]
fn subtree_run_does_not_emit_description_results_for_sibling_publishable_crates() {
    let root = temp_root("release-description-validation-scope");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml"])),
            ("crates", dir_entry(&["api", "other"], &[])),
            ("crates/api", dir_entry(&["src"], &["Cargo.toml"])),
            ("crates/api/src", dir_entry(&[], &["lib.rs"])),
            ("crates/other", dir_entry(&["src"], &["Cargo.toml"])),
            ("crates/other/src", dir_entry(&[], &["lib.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/api", "crates/other"]
resolver = "2"
"#,
            ),
            (
                "crates/api/Cargo.toml",
                r#"
[package]
name = "api"
version = "0.1.0"
edition = "2024"
license = "MIT"
repository = "https://example.com/api"
"#,
            ),
            ("crates/api/src/lib.rs", "pub fn api() {}\n"),
            (
                "crates/other/Cargo.toml",
                r#"
[package]
name = "other"
version = "0.1.0"
edition = "2024"
license = "MIT"
repository = "https://example.com/other"
"#,
            ),
            ("crates/other/src/lib.rs", "pub fn other() {}\n"),
        ],
        root,
    );

    let results = run_family(&tree, &StubToolChecker::new(true), false, "crates/api/src");

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            file: Some("crates/api/Cargo.toml"),
            inventory: Some(false),
            title_contains: Some("api: missing description"),
            ..Default::default()
        }],
    );
}
