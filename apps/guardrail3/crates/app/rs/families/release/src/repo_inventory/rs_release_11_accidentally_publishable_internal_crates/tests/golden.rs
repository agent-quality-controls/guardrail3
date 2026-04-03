use guardrail3_app_rs_family_release_assertions::repo_inventory::rs_release_11_accidentally_publishable_internal_crates as assertions;

use super::helpers::check;
use super::helpers::{crate_facts, crate_input};

#[test]
fn warns_on_publishable_crate_with_no_release_metadata() {
    let mut facts = crate_facts("internal");
    facts.description_present = false;
    facts.license_present = false;
    facts.repository_present = false;
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(false),
            title_contains: Some("internal"),
            message_contains: Some("internal"),
            ..Default::default()
        }],
    );
}
