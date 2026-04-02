use guardrail3_app_rs_family_release_assertions::repo_policy::rs_release_01_license_file as assertions;

use super::super::check;
use super::super::{repo_facts, repo_input};

#[test]
fn inventories_real_license_file_path() {
    let mut facts = repo_facts();
    facts.license_rel_path = Some("LICENSE-APACHE".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("LICENSE-APACHE"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_each_allowed_root_license_name() {
    for rel_path in ["LICENSE", "LICENSE-MIT", "LICENSE.md"] {
        let mut facts = repo_facts();
        facts.license_rel_path = Some(rel_path.to_owned());
        let input = repo_input(&facts);
        let mut results = Vec::new();

        check(&input, &mut results);

        assert!(!assertions::findings(&results).is_empty());
        assertions::assert_rule_results(
            &results,
            &[assertions::ExpectedRuleResult {
                file: Some(rel_path),
                inventory: Some(true),
                ..Default::default()
            }],
        );
    }
}
