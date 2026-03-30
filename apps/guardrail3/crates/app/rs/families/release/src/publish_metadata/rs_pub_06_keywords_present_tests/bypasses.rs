use guardrail3_app_rs_family_release_assertions::rs_pub_06_keywords_present as assertions;

use super::super::check;
use super::super::{crate_facts, crate_input};

#[test]
fn warns_when_keywords_are_missing_or_zero() {
    for keywords_count in [None, Some(0)] {
        let mut facts = crate_facts("x");
        facts.keywords_count = keywords_count;
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
                title_contains: Some("keywords missing"),
                message_contains: Some("1-5 `[package].keywords`"),
                ..Default::default()
            }],
        );
    }
}

#[test]
fn warns_when_too_many_keywords_are_present() {
    let mut facts = crate_facts("x");
    facts.keywords_count = Some(6);
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
            title_contains: Some("too many keywords"),
            message_contains: Some("at most 5"),
            ..Default::default()
        }],
    );
}

#[test]
fn skips_non_publishable_crates() {
    let mut facts = crate_facts("x");
    facts.publishable = false;
    facts.keywords_count = None;
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
}
