use crate::domain::report::Severity;

use super::super::test_support::{crate_facts, crate_input};
use super::check;

#[test]
fn warns_on_publishable_crate_with_no_release_metadata() {
    let mut facts = crate_facts("internal");
    facts.description_present = false;
    facts.license_present = false;
    facts.repository_present = false;
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}
