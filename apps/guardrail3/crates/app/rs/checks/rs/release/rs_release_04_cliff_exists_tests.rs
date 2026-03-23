use crate::domain::report::Severity;

use super::super::test_support::{repo_facts, repo_input};
use super::check;

#[test]
fn warns_when_cliff_missing() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn inventories_when_cliff_exists() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}
