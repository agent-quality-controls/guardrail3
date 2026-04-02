use guardrail3_app_rs_family_hooks_shared_assertions::bootstrap::hook_shared_02_hooks_path_configured as assertions;

use crate::hook_shared_02_hooks_path_configured::check;

#[test]
fn errors_when_hooks_path_missing() {
    let mut results = Vec::new();
    check(None, &mut results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("core.hooksPath not configured"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_hooks_path_is_wrong() {
    let mut results = Vec::new();
    check(Some(".git/hooks"), &mut results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("core.hooksPath has wrong value"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_expected_hooks_path() {
    let mut results = Vec::new();
    check(Some(".githooks"), &mut results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("core.hooksPath configured"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
