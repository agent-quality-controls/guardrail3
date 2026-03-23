use crate::domain::report::Severity;

use super::super::test_support::{lockfile_facts, lockfile_input};
use super::check;

#[test]
fn inventories_clean_gitignore() {
    let facts = lockfile_facts(true, false, Some("service"));
    let input = lockfile_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn errors_when_gitignore_ignores_cargo_lock() {
    let facts = lockfile_facts(true, true, Some("service"));
    let input = lockfile_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some(".gitignore"));
}

#[test]
fn collect_treats_wildcard_gitignore_as_ignoring_cargo_lock() {
    let tree = super::super::test_support::project_tree(
        vec![(
            "",
            super::super::test_support::dir_entry(&[], &[".gitignore", "Cargo.toml", "Cargo.lock"]),
        )],
        vec![
            ("Cargo.toml", "[package]\nname = \"crate\""),
            (".gitignore", "**/Cargo.lock"),
        ],
    );
    let facts = super::super::test_support::collected_facts(&tree, &[]);
    let input = lockfile_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
}
