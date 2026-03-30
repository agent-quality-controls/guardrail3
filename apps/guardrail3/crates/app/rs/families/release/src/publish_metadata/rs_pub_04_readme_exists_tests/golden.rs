use guardrail3_app_rs_family_release_assertions::rs_pub_04_readme_exists as assertions;

use super::super::check;
use super::super::{crate_facts, crate_input};

#[test]
fn inventories_existing_readme_for_publishable_crate() {
    let facts = crate_facts("example");
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("crates/example/README.md"),
            inventory: Some(true),
            message_contains: Some("README exists"),
            ..Default::default()
        }],
    );
}
