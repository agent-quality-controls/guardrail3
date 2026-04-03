use guardrail3_app_rs_family_release_assertions::publish_metadata::rs_pub_05_readme_quality as assertions;

use super::helpers::check;
use super::helpers::{crate_facts, crate_input};

#[test]
fn inventories_good_readme_quality_for_publishable_crate() {
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
            message_contains: Some("content and headings"),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_good_readme_quality_with_setext_heading() {
    let mut facts = crate_facts("example");
    facts.readme_content = Some(format!(
        "Example crate\n=============\n\n{}",
        "x".repeat(260)
    ));
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
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_good_readme_quality_with_multi_hash_heading() {
    let mut facts = crate_facts("example");
    facts.readme_content = Some(format!("## Example crate\n\n{}", "x".repeat(260)));
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
            ..Default::default()
        }],
    );
}
