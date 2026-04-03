use guardrail3_app_rs_family_release_assertions::publish_metadata::rs_pub_01_description_present as assertions;

use super::helpers::check;
use super::helpers::{crate_facts, crate_input};

#[test]
fn inventories_description_for_publishable_crate() {
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
            title_contains: Some("description"),
            message_contains: Some("[package].description"),
            ..Default::default()
        }],
    );
}
