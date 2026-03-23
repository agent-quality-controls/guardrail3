use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn warns_on_missing_or_too_many_keywords_and_skips_non_publishable_crates() {
    let mut missing = crate_facts("x");
    missing.keywords_count = None;
    let missing_input = crate_input(&missing);
    let mut missing_results = Vec::new();
    check(&missing_input, &mut missing_results);
    assert_eq!(missing_results[0].severity, Severity::Warn);

    let mut too_many = crate_facts("x");
    too_many.keywords_count = Some(6);
    let too_many_input = crate_input(&too_many);
    let mut too_many_results = Vec::new();
    check(&too_many_input, &mut too_many_results);
    assert_eq!(too_many_results[0].severity, Severity::Warn);

    let mut non_publishable = crate_facts("x");
    non_publishable.publishable = false;
    non_publishable.keywords_count = None;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(non_publishable_results.is_empty());
}
