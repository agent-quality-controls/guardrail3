use crate::app::rs::checks::rs::deps::test_support::{lockfile_facts, lockfile_input};
use crate::domain::report::Severity;

#[test]
fn inventories_present_cargo_lock() {
    let facts = lockfile_facts(true, false, Some("service"));
    let input = lockfile_input(&facts);
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DEPS-09");
    assert_eq!(result.severity, Severity::Info);
    assert!(result.inventory);
    assert_eq!(result.message, "Rust root `.` has `Cargo.lock` committed.");
}
