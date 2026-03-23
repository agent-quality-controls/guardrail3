use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn inventories_when_semver_checks_tool_is_installed() {
    let mut facts = repo_facts();
    facts.semver_checks_installed = true;
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-08");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}
