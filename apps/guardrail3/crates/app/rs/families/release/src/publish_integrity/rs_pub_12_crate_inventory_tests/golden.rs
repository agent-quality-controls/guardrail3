use guardrail3_app_rs_family_release_assertions::rs_pub_12_crate_inventory as assertions;

use super::super::check;
use super::super::{repo_facts, repo_input};

#[test]
fn inventories_publishable_and_non_publishable_counts() {
    let mut facts = repo_facts();
    facts.publishable_count = 2;
    facts.non_publishable_count = 1;
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("Crate inventory"),
            file: Some("Cargo.toml"),
            inventory: Some(true),
            message: Some("Repo has 2 publishable crate(s) and 1 non-publishable crate(s)."),
            ..Default::default()
        }],
    );
}
