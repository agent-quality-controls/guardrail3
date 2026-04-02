use guardrail3_app_rs_family_release_assertions::publish_metadata::rs_pub_06_keywords_present as assertions;

use super::super::check;
use super::super::{crate_facts, crate_input};

#[test]
fn inventories_valid_keywords_count() {
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
            message_contains: Some("3 entries"),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_keywords_at_lower_and_upper_allowed_boundaries() {
    for count in [1, 5] {
        let mut facts = crate_facts("x");
        facts.keywords_count = Some(count);
        let input = crate_input(&facts);
        let mut results = Vec::new();

        check(&input, &mut results);

        let expected_message = format!("{count} entries");
        assert!(!assertions::findings(&results).is_empty());
        assertions::assert_rule_results(
            &results,
            &[assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Info),
                file: Some("crates/example/Cargo.toml"),
                inventory: Some(true),
                message_contains: Some(expected_message.as_str()),
                ..Default::default()
            }],
        );
    }
}
