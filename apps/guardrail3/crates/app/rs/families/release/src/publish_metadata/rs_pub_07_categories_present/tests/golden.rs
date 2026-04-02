use guardrail3_app_rs_family_release_assertions::publish_metadata::rs_pub_07_categories_present as assertions;

use super::super::check;
use super::super::{crate_facts, crate_input};

#[test]
fn inventories_categories_when_present() {
    let facts = crate_facts("x");
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(true),
            message_contains: Some("[package].categories"),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_categories_at_lower_allowed_boundary() {
    let mut facts = crate_facts("x");
    facts.categories_count = Some(1);
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(true),
            message_contains: Some("1 entries"),
            ..Default::default()
        }],
    );
}
