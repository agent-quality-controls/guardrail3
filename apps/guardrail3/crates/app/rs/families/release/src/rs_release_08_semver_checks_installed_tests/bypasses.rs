use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;
use guardrail3_domain_report::Severity;

#[test]
fn warns_when_semver_checks_tool_is_missing() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-08");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("Cargo.toml"));
}
