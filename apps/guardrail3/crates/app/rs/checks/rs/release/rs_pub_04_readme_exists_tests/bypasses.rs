use crate::domain::report::Severity;

use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;

#[test]
fn warns_when_publishable_crate_has_no_readme_file() {
    let mut facts = crate_facts("example");
    facts.readme_exists = false;
    facts.readme_content = None;
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-04");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].title.contains("README missing"));
    assert!(results[0].message.contains("example"));
    assert!(results[0].message.contains("crates/example/README.md"));
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

    assert!(results.is_empty());
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

    assert!(results.is_empty());
}
