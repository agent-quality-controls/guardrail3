use guardrail3_app_rs_family_release_assertions::repo_policy::rs_release_02_release_plz_exists as assertions;

use super::helpers::check;
use super::helpers::{repo_facts, repo_input};

#[test]
fn inventories_release_plz_file_when_present() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("release-plz.toml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
