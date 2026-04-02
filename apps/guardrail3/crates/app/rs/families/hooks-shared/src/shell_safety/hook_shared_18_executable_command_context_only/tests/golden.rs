use guardrail3_app_rs_family_hooks_shared_assertions::shell_safety::hook_shared_18_executable_command_context_only as assertions;

use crate::hook_shared_18_executable_command_context_only::run_case;

#[test]
fn reports_guardrail_command_mentioned_only_in_comment() {
    let results = run_case("# guardrail3 rs validate --staged .\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("required hook step appears only in inert text"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn ignores_echo_or_comment_when_real_command_exists() {
    let results = run_case(
        r#"echo "guardrail3 rs validate --staged ."
guardrail3 rs validate --staged .
"#,
    );
    assertions::assert_rule_quiet(&results);
}

#[test]
fn still_reports_inert_guardrail_text_when_only_echo_exists() {
    let results = run_case(
        r#"# guardrail3 rs validate --staged .
echo "guardrail3 rs validate --staged ."
"#,
    );
    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Error),
                title: Some("required hook step appears only in inert text"),
                line: Some(1),
                inventory: Some(false),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Error),
                title: Some("required hook step appears only in inert text"),
                line: Some(2),
                inventory: Some(false),
                ..Default::default()
            },
        ],
    );
}
