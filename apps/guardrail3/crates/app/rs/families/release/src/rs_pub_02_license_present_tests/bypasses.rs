use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use guardrail3_domain_report::Severity;

#[test]
fn errors_without_license_metadata_and_skips_non_publishable_crates() {
    let mut missing = crate_facts("x");
    missing.license_present = false;
    let missing_input = crate_input(&missing);
    let mut missing_results = Vec::new();
    check(&missing_input, &mut missing_results);
    assert_eq!(missing_results.len(), 1);
    assert_eq!(missing_results[0].id, "RS-PUB-02");
    assert_eq!(missing_results[0].severity, Severity::Error);
    assert!(!missing_results[0].inventory);
    assert_eq!(
        missing_results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(missing_results[0].title.contains("license"));
    assert!(missing_results[0].message.contains("license"));

    let mut non_publishable = crate_facts("x");
    non_publishable.publishable = false;
    non_publishable.license_present = false;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(non_publishable_results.is_empty());
}
