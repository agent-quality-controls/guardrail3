use super::super::super::test_support::run_tree as run_family;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};
use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use guardrail3_domain_report::Severity;

#[test]
fn emits_info_when_include_exclude_is_missing_and_skips_non_publishable_crates() {
    let mut missing = crate_facts("x");
    missing.include_exclude_present = false;
    let missing_input = crate_input(&missing);
    let mut missing_results = Vec::new();
    check(&missing_input, &mut missing_results);
    assert_eq!(missing_results.len(), 1);
    assert_eq!(missing_results[0].id, "RS-PUB-14");
    assert_eq!(missing_results[0].severity, Severity::Info);
    assert!(!missing_results[0].inventory);
    assert_eq!(
        missing_results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(missing_results[0].title.contains("include/exclude missing"));
    assert!(
        missing_results[0]
            .message
            .contains("consider `include` or `exclude` patterns")
    );

    let mut non_publishable = crate_facts("x");
    non_publishable.publishable = false;
    non_publishable.include_exclude_present = false;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(non_publishable_results.is_empty());
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
        let tree = project_tree(
            vec![("", dir_entry(&[], &["Cargo.toml", "README.md"]))],
            vec![
                ("Cargo.toml", manifest),
                ("README.md", "# Readme\n\ncontent\n"),
            ],
            root,
        );
        let results = run_family(&tree, &StubToolChecker::new(true), false);

        assert!(results.iter().any(|result| {
            result.id == "RS-PUB-14"
                && result.severity == Severity::Info
                && !result.inventory
                && result.file.as_deref() == Some("Cargo.toml")
                && result.title.contains("include/exclude missing")
        }));
    }
}
