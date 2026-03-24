use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn inventories_release_profile_settings_when_present() {
    let mut facts = repo_facts();
    facts.release_profile_settings = vec!["lto = true".to_owned(), "strip = true".to_owned()];
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-10");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("Cargo.toml"));
    assert!(results[0].message.contains("lto = true"));
    assert!(results[0].message.contains("strip = true"));
}

#[test]
fn inventories_single_release_profile_setting_when_present() {
    let mut facts = repo_facts();
    facts.release_profile_settings = vec!["codegen-units = 1".to_owned()];
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-10");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("Cargo.toml"));
    assert!(results[0].message.contains("codegen-units = 1"));
}
