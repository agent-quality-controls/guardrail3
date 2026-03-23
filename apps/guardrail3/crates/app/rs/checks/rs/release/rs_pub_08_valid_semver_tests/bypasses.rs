use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn errors_on_invalid_semver_and_skips_non_publishable_crates() {
    let mut invalid = crate_facts("x");
    invalid.version_valid = false;
    invalid.version_string = Some("bad".to_owned());
    let invalid_input = crate_input(&invalid);
    let mut invalid_results = Vec::new();
    check(&invalid_input, &mut invalid_results);
    assert_eq!(invalid_results[0].severity, Severity::Error);

    let mut non_publishable = crate_facts("x");
    non_publishable.publishable = false;
    non_publishable.version_valid = false;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(non_publishable_results.is_empty());
}
