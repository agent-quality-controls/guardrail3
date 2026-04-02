use guardrail3_app_rs_family_hooks_shared_assertions::bootstrap::hook_shared_05_pre_commit_executable as assertions;

use crate::hook_shared_05_pre_commit_executable::check;

#[test]
fn inventories_executable_dispatcher() {
    let mut results = Vec::new();
    check(".githooks/pre-commit", Some(true), &mut results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("pre-commit hook is executable"),
            file: Some(".githooks/pre-commit"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_dispatcher_not_executable() {
    let mut results = Vec::new();
    check(".githooks/pre-commit", Some(false), &mut results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("pre-commit hook is not executable"),
            file: Some(".githooks/pre-commit"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_permissions_unavailable() {
    let mut results = Vec::new();
    check(".githooks/pre-commit", None, &mut results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("pre-commit hook permissions unavailable"),
            file: Some(".githooks/pre-commit"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
