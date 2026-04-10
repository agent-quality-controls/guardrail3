use g3rs_hooks_shared_source_checks_assertions::shell_safety::hook_shared_18_executable_command_context_only as assertions;

use super::run_case;

#[test]
fn reports_guardrail_command_mentioned_only_in_comment() {
    let results = run_case("# guardrail3 rs validate --staged .\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
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
fn ignores_inert_guardrail_text_when_env_wrapper_executes_real_command() {
    let results = run_case(
        r#"# guardrail3 rs validate --staged .
env -i guardrail3 rs validate --staged .
"#,
    );
    assertions::assert_rule_quiet(&results);
}

#[test]
fn ignores_inert_guardrail_text_when_path_qualified_command_executes() {
    let results = run_case(
        r#"# guardrail3 rs validate --staged .
/usr/local/bin/guardrail3 rs validate --staged .
"#,
    );
    assertions::assert_rule_quiet(&results);
}

#[test]
fn ignores_inert_guardrail_text_when_called_function_executes_real_command() {
    let results = run_case(
        r#"# guardrail3 rs validate --staged .
run_guardrail() {
    guardrail3 rs validate --staged .
}
run_guardrail
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
                severity: Some(assertions::G3Severity::Error),
                title: Some("required hook step appears only in inert text"),
                line: Some(1),
                inventory: Some(false),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::G3Severity::Error),
                title: Some("required hook step appears only in inert text"),
                line: Some(2),
                inventory: Some(false),
                ..Default::default()
            },
        ],
    );
}
