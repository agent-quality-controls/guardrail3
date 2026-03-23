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
        "Library profile project is missing `Cargo.lock`."
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
        "Non-library Rust project is missing `Cargo.lock`."
    );
}
