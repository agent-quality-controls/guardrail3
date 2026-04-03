use guardrail3_app_rs_family_release_assertions::repo_inventory::rs_release_09_publish_status_inventory as assertions;

use super::helpers::check;
use super::helpers::{repo_facts, repo_input};

#[test]
fn inventories_publish_status_when_present() {
    let mut facts = repo_facts();
    facts.publish_setting = Some("false".to_owned());
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
            message_contains: Some("publish = false"),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_non_boolean_publish_status_when_present() {
    let mut facts = repo_facts();
    facts.publish_setting = Some("[\"internal\"]".to_owned());
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
            message_contains: Some("publish = [\"internal\"]"),
            ..Default::default()
        }],
    );
}
