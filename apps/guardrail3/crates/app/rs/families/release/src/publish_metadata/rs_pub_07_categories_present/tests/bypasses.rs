use guardrail3_app_rs_family_release_assertions::publish_metadata::rs_pub_07_categories_present as assertions;

use super::helpers::check;
use super::helpers::{crate_facts, crate_input};

#[test]
fn warns_when_categories_are_missing_or_zero() {
    for categories_count in [None, Some(0)] {
        let mut facts = crate_facts("x");
        facts.categories_count = categories_count;
        let input = crate_input(&facts);
        let mut results = Vec::new();
        check(&input, &mut results);

        assert!(!assertions::findings(&results).is_empty());
        assertions::assert_rule_results(
            &results,
            &[assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                file: Some("crates/example/Cargo.toml"),
                inventory: Some(false),
                title_contains: Some("categories missing"),
                message_contains: Some("non-empty `[package].categories`"),
                ..Default::default()
            }],
        );
    }
}

#[test]
fn skips_non_publishable_crates() {
    let mut facts = crate_facts("x");
    facts.publishable = false;
    facts.categories_count = None;
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
}
