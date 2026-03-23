use crate::domain::report::Severity;

use super::super::test_support::{repo_facts, repo_input};
use super::check;

#[test]
fn errors_when_license_missing() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn inventories_existing_license() {
    let mut facts = repo_facts();
    facts.license_rel_path = Some("LICENSE".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}
