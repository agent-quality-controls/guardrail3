use super::super::{crate_facts, crate_input};
use super::super::check;
use guardrail3_domain_report::Severity;

#[test]
fn warns_when_keywords_are_missing_or_zero() {
    for keywords_count in [None, Some(0)] {
        let mut facts = crate_facts("x");
        facts.keywords_count = keywords_count;
        let input = crate_input(&facts);
        let mut results = Vec::new();

        check(&input, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "RS-PUB-06");
        assert_eq!(results[0].severity, Severity::Warn);
        assert!(!results[0].inventory);
        assert_eq!(
            results[0].file.as_deref(),
            Some("crates/example/Cargo.toml")
        );
        assert!(results[0].title.contains("keywords missing"));
        assert!(results[0].message.contains("1-5 `[package].keywords`"));
    }
}

#[test]
fn warns_when_too_many_keywords_are_present() {
    let mut facts = crate_facts("x");
    facts.keywords_count = Some(6);
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-06");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].title.contains("too many keywords"));
    assert!(results[0].message.contains("at most 5"));
}

#[test]
fn skips_non_publishable_crates() {
    let mut facts = crate_facts("x");
    facts.publishable = false;
    facts.keywords_count = None;
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}
