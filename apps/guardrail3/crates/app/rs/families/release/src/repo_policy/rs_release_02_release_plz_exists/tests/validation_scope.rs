use guardrail3_app_rs_family_release_assertions::repo_policy::rs_release_02_release_plz_exists as assertions;

use super::helpers::run_tree_with_validation_scope as run_family;
use super::helpers::{StubToolChecker, dir_entry, project_tree, temp_root};

#[test]
fn repo_global_release_plz_rule_still_emits_under_nested_validation_scope() {
    let root = temp_root("release-plz-validation-scope");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml"])),
            ("crates", dir_entry(&["api"], &[])),
            ("crates/api", dir_entry(&["src"], &["Cargo.toml"])),
            ("crates/api/src", dir_entry(&[], &["lib.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/api"]
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
description = "api"
license = "MIT"
repository = "https://example.com/api"
"#,
            ),
            ("crates/api/src/lib.rs", "pub fn api() {}\n"),
        ],
        root,
    );

    let results = run_family(&tree, &StubToolChecker::new(true), false, "crates/api/src");

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            file: Some("release-plz.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn non_workspace_subtree_stays_silent() {
    let root = temp_root("release-plz-non-workspace-scope");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["docs", "ws"], &[])),
            ("docs", dir_entry(&["guide"], &[])),
            ("docs/guide", dir_entry(&[], &["overview.md"])),
            ("ws", dir_entry(&["crates"], &["Cargo.toml"])),
            ("ws/crates", dir_entry(&["api"], &[])),
            ("ws/crates/api", dir_entry(&["src"], &["Cargo.toml"])),
            ("ws/crates/api/src", dir_entry(&[], &["lib.rs"])),
        ],
        vec![
            (
                "ws/Cargo.toml",
                r#"
[workspace]
members = ["crates/api"]
resolver = "2"
"#,
            ),
            (
                "ws/crates/api/Cargo.toml",
                r#"
[package]
name = "api"
version = "0.1.0"
edition = "2024"
description = "api"
license = "MIT"
repository = "https://example.com/api"
"#,
            ),
            ("ws/crates/api/src/lib.rs", "pub fn api() {}\n"),
            ("docs/guide/overview.md", "# Guide\n"),
        ],
        root,
    );

    let results = run_family(&tree, &StubToolChecker::new(true), false, "docs/guide");

    assert!(results.is_empty(), "{results:#?}");
}
