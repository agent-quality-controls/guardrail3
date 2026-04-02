use guardrail3_app_rs_family_release_assertions::publish_metadata::rs_pub_04_readme_exists as assertions;

use super::super::check;
use super::super::{crate_facts, crate_input};

#[test]
fn warns_when_publishable_crate_has_no_readme_file() {
    let mut facts = crate_facts("example");
    facts.readme_exists = false;
    facts.readme_content = None;
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
            title_contains: Some("README missing"),
            message_contains: Some("example"),
            ..Default::default()
        }],
    );
}

#[test]
fn skips_explicit_readme_false_instead_of_warning_on_default_readme_path() {
    let mut facts = crate_facts("example");
    facts.readme_declared_false = true;
    facts.readme_exists = false;
    facts.readme_content = None;
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
}

#[test]
fn does_not_emit_for_non_publishable_crates() {
    let mut facts = crate_facts("example");
    facts.publishable = false;
    facts.readme_exists = false;
    facts.readme_content = None;
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
}
