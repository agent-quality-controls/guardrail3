use crate::hook_shell::parse_script;
use guardrail3_domain_report::Severity;

use super::super::facts::HookScriptKind;
use super::super::inputs::ExecutableCommandContextInput;
use super::check;

#[test]
fn warns_when_set_e_only_appears_in_comment() {
    let content = "# set -euo pipefail\n";
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
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_set_e_only_appears_in_echo() {
    let content = "echo \"set -euo pipefail\"\n";
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
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_real_shell_error_handling_line_exists() {
    let content = "#!/usr/bin/env bash\nset -euo pipefail\n";
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
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(results[0].inventory);
}
