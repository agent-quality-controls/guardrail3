use crate::domain::report::Severity;

use super::check;
use super::super::test_support::coverage_input;

#[test]
fn errors_when_no_tests_exist() {
    let input = coverage_input(false, 3, 0, false);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].id, "RS-TEST-04");
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn inventories_when_tests_exist() {
    let input = coverage_input(true, 3, 2, false);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}
