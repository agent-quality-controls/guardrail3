use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn inventories_publish_status_when_present() {
    let mut facts = repo_facts();
    facts.publish_setting = Some("false".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-09");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("Cargo.toml"));
    assert!(results[0].message.contains("publish = false"));
}

#[test]
fn inventories_non_boolean_publish_status_when_present() {
    let mut facts = repo_facts();
    facts.publish_setting = Some("[\"internal\"]".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-09");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("Cargo.toml"));
    assert!(results[0].message.contains("publish = [\"internal\"]"));
}
