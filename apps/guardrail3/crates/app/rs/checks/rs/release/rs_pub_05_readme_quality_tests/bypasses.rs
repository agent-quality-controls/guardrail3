use crate::domain::report::Severity;

use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;

#[test]
fn warns_on_stub_readme() {
    let mut facts = crate_facts("example");
    facts.readme_content = Some("# x\nshort".to_owned());
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-05");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("crates/example/README.md"));
    assert!(results[0].title.contains("stub"));
    assert!(results[0].message.contains("README.md"));
    assert!(results[0].message.contains("bytes"));
}

#[test]
fn warns_when_readme_has_no_heading() {
    let mut facts = crate_facts("example");
    facts.readme_content = Some("x".repeat(260));
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-05");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("crates/example/README.md"));
    assert!(results[0].title.contains("no heading"));
    assert!(results[0].message.contains("no markdown heading"));
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

    assert!(false_results.is_empty());

    let mut non_publishable = crate_facts("example");
    non_publishable.publishable = false;
    non_publishable.readme_content = Some("# x\nshort".to_owned());
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();

    check(&non_publishable_input, &mut non_publishable_results);

    assert!(non_publishable_results.is_empty());
}

#[test]
fn skips_missing_readme_and_leaves_that_to_readme_exists_rule() {
    let mut facts = crate_facts("example");
    facts.readme_exists = false;
    facts.readme_content = None;
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-05");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("crates/example/README.md"));
    assert!(results[0].title.contains("no heading"));
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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-05");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("crates/example/README.md"));
    assert!(results[0].title.contains("no heading"));
}

#[test]
fn warns_when_hash_prefixed_text_is_not_a_real_markdown_heading() {
    let mut facts = crate_facts("example");
    facts.readme_content = Some(format!("#Heading\n\n{}", "x".repeat(260)));
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-05");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("crates/example/README.md"));
    assert!(results[0].title.contains("no heading"));
}
