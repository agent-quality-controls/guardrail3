use guardrail3_app_rs_family_release_assertions::rs_release_10_release_profile_inventory as assertions;

use super::super::check;
use super::super::{repo_facts, repo_input};

#[test]
fn inventories_release_profile_settings_when_present() {
    let mut facts = repo_facts();
    facts.release_profile_settings = vec!["lto = true".to_owned(), "strip = true".to_owned()];
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("Cargo.toml"),
            inventory: Some(true),
            message_contains: Some("lto = true"),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_single_release_profile_setting_when_present() {
    let mut facts = repo_facts();
    facts.release_profile_settings = vec!["codegen-units = 1".to_owned()];
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("Cargo.toml"),
            inventory: Some(true),
            message_contains: Some("codegen-units = 1"),
            ..Default::default()
        }],
    );
}
