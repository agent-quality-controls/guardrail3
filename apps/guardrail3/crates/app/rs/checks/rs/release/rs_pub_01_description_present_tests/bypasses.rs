use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn errors_without_description_and_skips_non_publishable_crates() {
    let mut missing = crate_facts("x");
    missing.description_present = false;
    let missing_input = crate_input(&missing);
    let mut missing_results = Vec::new();
    check(&missing_input, &mut missing_results);
    assert_eq!(missing_results.len(), 1);
    assert_eq!(missing_results[0].id, "RS-PUB-01");
    assert_eq!(missing_results[0].severity, Severity::Error);
    assert!(!missing_results[0].inventory);

    let mut non_publishable = crate_facts("x");
    non_publishable.publishable = false;
    non_publishable.description_present = false;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(non_publishable_results.is_empty());
}
