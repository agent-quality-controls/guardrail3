use guardrail3_app_rs_family_release_assertions::rs_pub_05_readme_quality as assertions;

use super::super::check;
use super::super::{crate_facts, crate_input};

#[test]
fn warns_on_stub_readme() {
    let mut facts = crate_facts("example");
    facts.readme_content = Some("# x\nshort".to_owned());
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("stub"),
            file: Some("crates/example/README.md"),
            inventory: Some(false),
            message_contains: Some("README.md"),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_readme_has_no_heading() {
    let mut facts = crate_facts("example");
    facts.readme_content = Some("x".repeat(260));
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("no heading"),
            file: Some("crates/example/README.md"),
            inventory: Some(false),
            message_contains: Some("no markdown heading"),
            ..Default::default()
        }],
    );
}

#[test]
fn skips_explicit_readme_false_and_non_publishable_crates() {
    let mut false_opt_out = crate_facts("example");
    false_opt_out.readme_declared_false = true;
    false_opt_out.readme_exists = false;
    false_opt_out.readme_content = None;
    let false_input = crate_input(&false_opt_out);
    let mut false_results = Vec::new();

    check(&false_input, &mut false_results);

    assert!(assertions::findings(&false_results).is_empty());
    assertions::assert_rule_quiet(&false_results);

    let mut non_publishable = crate_facts("example");
    non_publishable.publishable = false;
    non_publishable.readme_content = Some("# x\nshort".to_owned());
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();

    check(&non_publishable_input, &mut non_publishable_results);

    assert!(assertions::findings(&non_publishable_results).is_empty());
    assertions::assert_rule_quiet(&non_publishable_results);
}

#[test]
fn skips_missing_readme_and_leaves_that_to_readme_exists_rule() {
    let mut facts = crate_facts("example");
    facts.readme_exists = false;
    facts.readme_content = None;
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
}

#[test]
fn warns_when_only_code_blocks_contain_hash_prefixes() {
    let mut facts = crate_facts("example");
    facts.readme_content = Some(format!(
        "Intro text\n\n```\n# not a heading\n```\n\n{}",
        "x".repeat(240)
    ));
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("no heading"),
            file: Some("crates/example/README.md"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_only_indented_code_blocks_contain_hash_prefixes() {
    let mut facts = crate_facts("example");
    facts.readme_content = Some(format!(
        "Intro text\n\n    # not a heading\n\n{}",
        "x".repeat(240)
    ));
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("no heading"),
            file: Some("crates/example/README.md"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_hash_prefixed_text_is_not_a_real_markdown_heading() {
    let mut facts = crate_facts("example");
    facts.readme_content = Some(format!("#Heading\n\n{}", "x".repeat(260)));
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("no heading"),
            file: Some("crates/example/README.md"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
