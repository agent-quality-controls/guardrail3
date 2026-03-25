use crate::hook_shell::parse_script;
use guardrail3_domain_report::Severity;

use super::super::facts::HookScriptKind;
use super::super::inputs::ExecutableCommandContextInput;
use super::check;

#[test]
fn warns_when_exit_zero_is_executable() {
    let content = "exit 0\n";
    let parsed = parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].line, Some(1));
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_exit_zero_only_appears_in_comment() {
    let content = "# exit 0\n";
    let parsed = parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}
