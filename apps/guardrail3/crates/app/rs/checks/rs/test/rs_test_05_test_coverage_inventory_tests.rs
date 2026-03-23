use crate::domain::report::Severity;

use super::super::test_support::coverage_input;
use super::check;

#[test]
fn inventories_ratio() {
    let input = coverage_input(true, 2, 1, false);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].id, "RS-TEST-05");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert!(results[0].message.contains("2 public functions"));
    assert!(results[0].message.contains("1 test functions"));
}
