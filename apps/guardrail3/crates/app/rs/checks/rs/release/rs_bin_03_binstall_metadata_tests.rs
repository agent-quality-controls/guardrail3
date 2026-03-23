use crate::domain::report::Severity;

use super::super::test_support::{crate_facts, crate_input};
use super::check;

#[test]
fn warns_without_binstall_metadata() {
    let mut facts = crate_facts("bin");
    facts.is_binary = true;
    facts.has_binstall_metadata = false;
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn inventories_binstall_metadata_when_present() {
    let mut facts = crate_facts("bin");
    facts.is_binary = true;
    facts.has_binstall_metadata = true;
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}
