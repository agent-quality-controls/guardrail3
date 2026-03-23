use crate::domain::report::Severity;

use super::super::test_support::{lockfile_facts, lockfile_input};
use super::check;

#[test]
fn inventories_present_cargo_lock() {
    let facts = lockfile_facts(true, false, Some("service"));
    let input = lockfile_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].message,
        "Rust root `.` has `Cargo.lock` committed."
    );
}

#[test]
fn missing_lock_is_info_for_library_profile() {
    let facts = lockfile_facts(false, false, Some("library"));
    let input = lockfile_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Info);
    assert_eq!(
        results[0].message,
        "Library-profile Rust root `.` is missing `Cargo.lock`."
    );
}

#[test]
fn missing_lock_is_error_for_non_library_profile() {
    let facts = lockfile_facts(false, false, Some("service"));
    let input = lockfile_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].message,
        "Non-library Rust root `.` is missing `Cargo.lock`."
    );
}

#[test]
fn collect_checks_nested_workspace_lockfile() {
    let tree = super::super::test_support::project_tree(
        vec![
            (
                "",
                super::super::test_support::dir_entry(&["apps"], &["guardrail3.toml"]),
            ),
            ("apps", super::super::test_support::dir_entry(&["api"], &[])),
            (
                "apps/api",
                super::super::test_support::dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.apps.api]
                    profile = "service"
                "#,
            ),
            (
                "apps/api/Cargo.toml",
                r#"
                    [workspace]
                    members = []
                "#,
            ),
        ],
    );
    let facts = super::super::test_support::collected_facts(&tree, &[]);
    let lockfile = facts
        .lockfiles
        .iter()
        .find(|lockfile| lockfile.root_rel_dir == "apps/api")
        .expect("expected nested workspace lockfile facts");
    let input = super::super::inputs::LockfileDepsInput::new(lockfile);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].message,
        "Non-library Rust root `apps/api` is missing `apps/api/Cargo.lock`."
    );
}
