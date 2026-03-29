use guardrail3_app_rs_family_hooks_shared_assertions::hook_shared_03_modular_directory_inventory as assertions;

use crate::hook_shared_03_modular_directory_inventory::check;

#[test]
fn inventories_modular_directory_when_present() {
    let mut results = Vec::new();
    check(true, &mut results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("pre-commit.d directory exists"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_monolithic_mode_when_missing() {
    let mut results = Vec::new();
    check(false, &mut results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("pre-commit.d directory missing"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
