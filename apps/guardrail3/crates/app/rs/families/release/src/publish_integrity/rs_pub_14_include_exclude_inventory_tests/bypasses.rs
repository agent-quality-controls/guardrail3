use guardrail3_app_rs_family_release_assertions::rs_pub_14_include_exclude_inventory as assertions;

use super::super::check;
use super::super::run_tree as run_family;
use super::super::{StubToolChecker, dir_entry, project_tree, temp_root};
use super::super::{crate_facts, crate_input};

#[test]
fn emits_info_when_include_exclude_is_missing_and_skips_non_publishable_crates() {
    let mut missing = crate_facts("x");
    missing.include_exclude_present = false;
    let missing_input = crate_input(&missing);
    let mut missing_results = Vec::new();
    check(&missing_input, &mut missing_results);
    assert!(!assertions::findings(&missing_results).is_empty());
    assertions::assert_rule_results(
        &missing_results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(false),
            title_contains: Some("include/exclude missing"),
            message_contains: Some("consider `include` or `exclude` patterns"),
            ..Default::default()
        }],
    );

    let mut non_publishable = crate_facts("x");
    non_publishable.publishable = false;
    non_publishable.include_exclude_present = false;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(assertions::findings(&non_publishable_results).is_empty());
    assertions::assert_rule_quiet(&non_publishable_results);
}

#[test]
fn treats_empty_include_or_exclude_lists_as_missing() {
    for manifest in [
        r#"
[package]
name = "crate-a"
version = "0.1.0"
edition = "2024"
description = "crate-a"
license = "MIT"
repository = "https://example.com/a"
include = []
"#,
        r#"
[package]
name = "crate-b"
version = "0.1.0"
edition = "2024"
description = "crate-b"
license = "MIT"
repository = "https://example.com/b"
exclude = []
"#,
    ] {
        let root = temp_root("release-empty-include-exclude");
        let manifest = format!("[workspace]\nresolver = \"2\"\n\n{manifest}");
        let tree = project_tree(
            vec![("", dir_entry(&[], &["Cargo.toml", "README.md"]))],
            vec![("Cargo.toml", manifest.as_str()), ("README.md", "# Readme\n\ncontent\n")],
            root,
        );
        let results = run_family(&tree, &StubToolChecker::new(true), false);

        assert!(!assertions::findings(&results).is_empty());
        assertions::assert_rule_results(
            &results,
            &[assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Info),
                file: Some("Cargo.toml"),
                inventory: Some(false),
                title_contains: Some("include/exclude missing"),
                ..Default::default()
            }],
        );
    }
}
