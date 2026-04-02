use guardrail3_app_rs_family_hooks_shared_assertions::shell_safety::hook_shared_14_no_bypass_instructions as assertions;

use crate::hook_shared_14_no_bypass_instructions::run_case;

#[test]
fn flags_comment_teaching_no_verify() {
    let results = run_case("# use git commit --no-verify if this gets in the way\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("hook bypass instructions present"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_no_no_verify_comment_exists() {
    let results = run_case("# normal comment\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("no hook bypass instructions"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
