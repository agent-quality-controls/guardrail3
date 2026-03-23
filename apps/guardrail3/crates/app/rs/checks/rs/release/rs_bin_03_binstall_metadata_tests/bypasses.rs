use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn warns_without_binstall_metadata_and_skips_out_of_scope_crates() {
    let mut missing = crate_facts("bin");
    missing.is_binary = true;
    missing.has_binstall_metadata = false;
    let missing_input = crate_input(&missing);
    let mut missing_results = Vec::new();
    check(&missing_input, &mut missing_results);
    assert_eq!(missing_results.len(), 1);
    assert_eq!(missing_results[0].id, "RS-BIN-03");
    assert_eq!(missing_results[0].severity, Severity::Warn);
    assert!(!missing_results[0].inventory);

    let library = crate_facts("lib");
    let library_input = crate_input(&library);
    let mut library_results = Vec::new();
    check(&library_input, &mut library_results);
    assert!(library_results.is_empty());

    let mut non_publishable = crate_facts("bin");
    non_publishable.is_binary = true;
    non_publishable.publishable = false;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(non_publishable_results.is_empty());
}
