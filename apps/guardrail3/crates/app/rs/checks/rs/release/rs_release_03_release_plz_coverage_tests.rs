use crate::domain::report::Severity;

use super::super::test_support::{repo_facts, repo_input};
use super::check;

#[test]
fn warns_when_workspace_section_missing() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(toml::Value::Table(Default::default()));
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn inventories_when_release_plz_covers_publishable_crates() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(toml::Value::Table(Default::default()));
    facts.release_plz_has_workspace = true;
    let _ = facts.publishable_crate_names.insert("api".to_owned());
    let _ = facts.release_plz_package_names.insert("api".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}
