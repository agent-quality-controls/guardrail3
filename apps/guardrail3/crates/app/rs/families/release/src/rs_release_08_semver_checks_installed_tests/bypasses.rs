use guardrail3_app_rs_family_release_assertions::rs_release_08_semver_checks_installed as assertions;

use super::super::check;
use super::super::{repo_facts, repo_input};

#[test]
fn warns_when_semver_checks_tool_is_missing() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            file: Some("Cargo.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
