use guardrail3_app_rs_family_release_assertions::rs_pub_12_crate_inventory as assertions;
use super::super::{repo_facts, repo_input};
use super::super::check;

#[test]
fn inventories_zero_publishable_and_zero_non_publishable_counts() {
    let mut facts = repo_facts();
    facts.publishable_count = 0;
    facts.non_publishable_count = 0;
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
            message: Some("Repo has 0 publishable crate(s) and 0 non-publishable crate(s)."),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_when_only_non_publishable_crates_exist() {
    let mut facts = repo_facts();
    facts.publishable_count = 0;
    facts.non_publishable_count = 3;
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
            message: Some("Repo has 0 publishable crate(s) and 3 non-publishable crate(s)."),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_when_only_publishable_crates_exist() {
    let mut facts = repo_facts();
    facts.publishable_count = 4;
    facts.non_publishable_count = 0;
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
            message: Some("Repo has 4 publishable crate(s) and 0 non-publishable crate(s)."),
            ..Default::default()
        }],
    );
}
