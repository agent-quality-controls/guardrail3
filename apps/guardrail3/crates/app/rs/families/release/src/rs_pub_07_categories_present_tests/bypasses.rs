use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use guardrail3_domain_report::Severity;

#[test]
fn warns_when_categories_are_missing_or_zero() {
    for categories_count in [None, Some(0)] {
        let mut facts = crate_facts("x");
        facts.categories_count = categories_count;
        let input = crate_input(&facts);
        let mut results = Vec::new();
        check(&input, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "RS-PUB-07");
        assert_eq!(results[0].severity, Severity::Warn);
        assert!(!results[0].inventory);
        assert_eq!(
            results[0].file.as_deref(),
            Some("crates/example/Cargo.toml")
        );
        assert!(results[0].title.contains("categories missing"));
        assert!(
            results[0]
                .message
                .contains("non-empty `[package].categories`")
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

    assert!(results.is_empty());
}
