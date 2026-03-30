use guardrail3_app_rs_family_release_assertions::rs_release_08_semver_checks_installed as assertions;

use super::super::check;
use super::super::{repo_facts, repo_input};

#[test]
fn inventories_when_semver_checks_tool_is_installed() {
    let mut facts = repo_facts();
    facts.semver_checks_installed = true;
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
            ..Default::default()
        }],
    );
}
