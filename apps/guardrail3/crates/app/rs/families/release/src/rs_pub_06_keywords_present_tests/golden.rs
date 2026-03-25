use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use guardrail3_domain_report::Severity;

#[test]
fn inventories_valid_keywords_count() {
    let facts = crate_facts("x");
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-06");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].message.contains("3 entries"));
}

#[test]
fn inventories_keywords_at_lower_and_upper_allowed_boundaries() {
    for count in [1, 5] {
        let mut facts = crate_facts("x");
        facts.keywords_count = Some(count);
        let input = crate_input(&facts);
        let mut results = Vec::new();

        check(&input, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "RS-PUB-06");
        assert_eq!(results[0].severity, Severity::Info);
        assert!(results[0].inventory);
        assert_eq!(
            results[0].file.as_deref(),
            Some("crates/example/Cargo.toml")
        );
        assert!(results[0].message.contains(&format!("{count} entries")));
    }
}
