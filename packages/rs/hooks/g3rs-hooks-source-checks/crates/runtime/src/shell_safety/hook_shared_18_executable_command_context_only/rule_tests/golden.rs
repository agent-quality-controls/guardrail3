use g3rs_hooks_source_checks_assertions::shell_safety::hook_shared_18_executable_command_context_only::rule as assertions;

use super::run_case;

#[test]
fn reports_guardrail_command_mentioned_only_in_comment() {
    let results = run_case("# g3rs rs validate --staged .\n");
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
fn reports_guardrail_command_mentioned_only_in_assignment_text() {
    let results = run_case("VALIDATE_CMD='g3rs rs validate --staged .'\n");
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
fn reports_guardrail_command_mentioned_only_in_heredoc_body() {
    let results = run_case("cat <<'EOF'\ng3rs rs validate --staged .\nEOF\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            title: Some("required hook step appears only in inert text"),
            line: Some(2),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn ignores_echo_or_comment_when_real_command_exists() {
    let results = run_case(
        r#"echo "g3rs rs validate --staged ."
g3rs rs validate --staged .
"#,
    );
    assertions::assert_rule_quiet(&results);
}

#[test]
fn ignores_inert_guardrail_text_when_env_wrapper_executes_real_command() {
    let results = run_case(
        r#"# g3rs rs validate --staged .
env -i g3rs rs validate --staged .
"#,
    );
    assertions::assert_rule_quiet(&results);
}

#[test]
fn ignores_inert_guardrail_text_when_path_qualified_command_executes() {
    let results = run_case(
        r#"# g3rs rs validate --staged .
/usr/local/bin/g3rs rs validate --staged .
"#,
    );
    assertions::assert_rule_quiet(&results);
}

#[test]
fn ignores_inert_guardrail_text_when_called_function_executes_real_command() {
    let results = run_case(
        r#"# g3rs rs validate --staged .
run_guardrail() {
    g3rs rs validate --staged .
}
run_guardrail
"#,
    );
    assertions::assert_rule_quiet(&results);
}

#[test]
fn still_reports_inert_guardrail_text_when_only_echo_exists() {
    let results = run_case(
        r#"# g3rs rs validate --staged .
echo "g3rs rs validate --staged ."
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
