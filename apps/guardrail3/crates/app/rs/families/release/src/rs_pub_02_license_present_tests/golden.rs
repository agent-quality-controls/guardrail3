use guardrail3_app_rs_family_release_assertions::rs_pub_02_license_present as assertions;

use super::super::check;
use super::super::{crate_facts, crate_input};

#[test]
fn inventories_license_metadata_for_publishable_crate() {
    let facts = crate_facts("x");
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(true),
            title_contains: Some("license"),
            message_contains: Some("license"),
            ..Default::default()
        }],
    );
}
