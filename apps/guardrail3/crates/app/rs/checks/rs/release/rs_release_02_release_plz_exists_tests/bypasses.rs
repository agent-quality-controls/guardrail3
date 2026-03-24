use crate::domain::report::Severity;

use super::super::super::check as run_family;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};
use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;

#[test]
fn warns_when_release_plz_file_is_missing() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-02");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
}

#[test]
fn nested_non_root_release_plz_file_does_not_satisfy_repo_root_rule() {
    let root = temp_root("release-plz-nested-non-root");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["examples"], &["Cargo.toml"])),
            ("examples", dir_entry(&["demo"], &[])),
            ("examples/demo", dir_entry(&[], &["release-plz.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "example"
version = "0.1.0"
description = "example"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            ("examples/demo/release-plz.toml", "[workspace]\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-RELEASE-02"
            && result.severity == Severity::Warn
            && !result.inventory
            && result.file.as_deref() == Some("release-plz.toml")
    }));
}

#[test]
fn malformed_root_release_plz_still_satisfies_existence_rule() {
    let root = temp_root("release-plz-root-malformed-exists");
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "release-plz.toml"]))],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "example"
version = "0.1.0"
description = "example"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            ("release-plz.toml", "[workspace"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-RELEASE-02"
            && result.severity == Severity::Info
            && result.inventory
            && result.file.as_deref() == Some("release-plz.toml")
    }));
    assert!(
        !results
            .iter()
            .any(|result| { result.id == "RS-RELEASE-02" && result.severity == Severity::Warn })
    );
    assert!(results.iter().any(|result| {
        result.id == "RS-RELEASE-12"
            && result.severity == Severity::Error
            && result.file.as_deref() == Some("release-plz.toml")
    }));
}
