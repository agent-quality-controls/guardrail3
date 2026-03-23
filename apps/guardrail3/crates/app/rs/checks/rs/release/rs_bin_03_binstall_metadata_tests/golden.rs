use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn inventories_binstall_metadata_for_publishable_binary_crate() {
    let mut facts = crate_facts("bin");
    facts.is_binary = true;
    facts.has_binstall_metadata = true;
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-03");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}
