use guardrail3_app_rs_family_hooks_shared_assertions::bootstrap::hook_shared_01_pre_commit_exists as assertions;

use crate::hook_shared_01_pre_commit_exists::run_case;

#[test]
fn errors_when_pre_commit_is_missing() {
    let results = run_case(None);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("pre-commit hook missing"),
            file: Some(".githooks/pre-commit"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_existing_pre_commit_script() {
    let results = run_case(Some("#!/usr/bin/env bash\n"));
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("pre-commit hook exists"),
            file: Some(".githooks/pre-commit"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
