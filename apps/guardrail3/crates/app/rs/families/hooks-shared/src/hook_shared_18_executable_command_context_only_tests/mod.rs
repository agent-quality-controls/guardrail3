use crate::hook_shell::parse_script;
use guardrail3_domain_report::Severity;

use super::super::facts::HookScriptKind;
use super::super::hook_shared_18_executable_command_context_only::check;
use super::super::inputs::ExecutableCommandContextInput;

#[test]
fn reports_guardrail_command_mentioned_only_in_comment() {
    let content = "# guardrail3 rs validate --staged .\n";
    let parsed = parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };

    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "HOOK-SHARED-18");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].line, Some(1));
}

#[test]
fn ignores_echo_or_comment_when_real_command_exists() {
    let content = r#"echo "guardrail3 rs validate --staged ."
guardrail3 rs validate --staged .
"#;
    let parsed = parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };

    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(results.is_empty());
}

#[test]
fn still_reports_inert_guardrail_text_when_only_echo_exists() {
    let content = r#"# guardrail3 rs validate --staged .
echo "guardrail3 rs validate --staged ."
"#;
    let parsed = parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };

    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|result| result.id == "HOOK-SHARED-18"));
}
