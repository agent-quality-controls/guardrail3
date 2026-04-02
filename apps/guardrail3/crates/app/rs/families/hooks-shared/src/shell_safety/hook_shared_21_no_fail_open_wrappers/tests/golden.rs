use guardrail3_app_rs_family_hooks_shared_assertions::shell_safety::hook_shared_21_no_fail_open_wrappers as assertions;

use crate::hook_shared_21_no_fail_open_wrappers::run_case;

#[test]
fn reports_fail_open_wrapper_on_critical_command() {
    let results = run_case("guardrail3 rs validate --staged . || true\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title: Some("critical hook command is fail-open"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn ignores_fail_open_wrapper_on_non_critical_command() {
    let results = run_case("grep -q needle file || true\n");
    assertions::assert_rule_quiet(&results);
}

#[test]
fn ignores_echoed_critical_command_with_literal_fail_open_text() {
    let results = run_case("echo \"guardrail3 rs validate --staged . || true\"\n");
    assertions::assert_rule_quiet(&results);
}
