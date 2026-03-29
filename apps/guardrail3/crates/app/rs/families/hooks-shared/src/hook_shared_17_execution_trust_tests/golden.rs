use guardrail3_app_rs_family_hooks_shared_assertions::hook_shared_17_execution_trust as assertions;

use crate::hook_shared_17_execution_trust::check;

#[test]
fn inventories_clean_trust_state() {
    let mut results = Vec::new();
    check(&[], &mut results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title: Some("no competing hook systems detected"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_competing_hook_systems_exist() {
    let mut results = Vec::new();
    check(
        &[
            ".husky/pre-commit".to_owned(),
            ".git/hooks/pre-commit".to_owned(),
        ],
        &mut results,
    );
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title: Some("competing hook system detected"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
    assertions::assert_message_contains(&results, ".husky/pre-commit");
}
