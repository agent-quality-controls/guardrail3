use guardrail3_app_rs_family_release_assertions::rs_pub_02_license_present as assertions;

use super::super::{crate_facts, crate_input};
use super::super::check;

#[test]
fn errors_without_license_metadata_and_skips_non_publishable_crates() {
    let mut missing = crate_facts("x");
    missing.license_present = false;
    let missing_input = crate_input(&missing);
    let mut missing_results = Vec::new();
    check(&missing_input, &mut missing_results);
    assert!(!assertions::findings(&missing_results).is_empty());
    assertions::assert_rule_results(
        &missing_results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(false),
            title_contains: Some("license"),
            message_contains: Some("license"),
            ..Default::default()
        }],
    );

    let mut non_publishable = crate_facts("x");
    non_publishable.publishable = false;
    non_publishable.license_present = false;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(assertions::findings(&non_publishable_results).is_empty());
    assertions::assert_rule_quiet(&non_publishable_results);
}
