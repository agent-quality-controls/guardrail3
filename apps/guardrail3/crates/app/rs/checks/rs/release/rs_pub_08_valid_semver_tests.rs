use crate::domain::report::Severity;

use super::super::test_support::{crate_facts, crate_input};
use super::check;

#[test]
fn errors_on_invalid_version() {
    let mut facts = crate_facts("x");
    facts.version_valid = false;
    facts.version_string = Some("bad".to_owned());
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn inventories_workspace_version() {
    let mut facts = crate_facts("x");
    facts.workspace_version = true;
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}
