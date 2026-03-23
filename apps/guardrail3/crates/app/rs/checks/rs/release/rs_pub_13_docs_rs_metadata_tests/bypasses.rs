use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn emits_info_when_docs_rs_metadata_is_missing_and_skips_out_of_scope_crates() {
    let mut missing = crate_facts("x");
    missing.docs_rs_present = false;
    let missing_input = crate_input(&missing);
    let mut missing_results = Vec::new();
    check(&missing_input, &mut missing_results);
    assert_eq!(missing_results.len(), 1);
    assert_eq!(missing_results[0].id, "RS-PUB-13");
    assert_eq!(missing_results[0].severity, Severity::Info);
    assert!(!missing_results[0].inventory);

    let mut binary = crate_facts("bin");
    binary.is_library = false;
    binary.docs_rs_present = false;
    let binary_input = crate_input(&binary);
    let mut binary_results = Vec::new();
    check(&binary_input, &mut binary_results);
    assert!(binary_results.is_empty());

    let mut non_publishable = crate_facts("x");
    non_publishable.publishable = false;
    non_publishable.docs_rs_present = false;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(non_publishable_results.is_empty());
}
