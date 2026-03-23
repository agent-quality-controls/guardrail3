use crate::domain::report::Severity;

use super::check;
use super::super::test_support::coverage_input;

#[test]
fn info_when_missing_integration_tests() {
    let input = coverage_input(true, 1, 1, false);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].id, "RS-TEST-06");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(!results[0].inventory);
}

#[test]
fn inventories_when_integration_tests_exist() {
    let input = coverage_input(true, 1, 1, true);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}
