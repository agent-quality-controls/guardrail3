use crate::domain::report::Severity;

use super::super::test_support::module_input;
use super::check;

#[test]
fn warns_on_non_tests_module_name() {
    let input = module_input("src/lib.rs", "test_utils", true);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn accepts_tests_module_name() {
    let input = module_input("src/lib.rs", "tests", true);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.is_empty());
}
