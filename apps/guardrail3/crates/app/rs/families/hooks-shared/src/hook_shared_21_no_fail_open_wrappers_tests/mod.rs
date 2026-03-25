use crate::hook_shell::parse_script;
use guardrail3_domain_report::Severity;

use super::super::hook_shared_21_no_fail_open_wrappers::check;
use super::super::inputs::FailOpenWrapperInput;

#[test]
fn reports_fail_open_wrapper_on_critical_command() {
    let content = "guardrail3 rs validate --staged . || true\n";
    let parsed = parse_script(content);
    let input = FailOpenWrapperInput {
        rel_path: ".githooks/pre-commit",
        executable_lines: &parsed.executable_lines,
    };

    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "HOOK-SHARED-21");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].line, Some(1));
}

#[test]
fn ignores_fail_open_wrapper_on_non_critical_command() {
    let content = "grep -q needle file || true\n";
    let parsed = parse_script(content);
    let input = FailOpenWrapperInput {
        rel_path: ".githooks/pre-commit",
        executable_lines: &parsed.executable_lines,
    };

    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(results.is_empty());
}

#[test]
fn ignores_echoed_critical_command_with_literal_fail_open_text() {
    let content = "echo \"guardrail3 rs validate --staged . || true\"\n";
    let parsed = parse_script(content);
    let input = FailOpenWrapperInput {
        rel_path: ".githooks/pre-commit",
        executable_lines: &parsed.executable_lines,
    };

    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(results.is_empty());
}
