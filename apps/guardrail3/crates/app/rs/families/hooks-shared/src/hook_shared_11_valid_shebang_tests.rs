use crate::hook_shell::parse_script;
use guardrail3_domain_report::Severity;

use super::super::facts::HookScriptKind;
use super::super::inputs::ExecutableCommandContextInput;
use super::check;

#[test]
fn warns_when_shebang_is_missing() {
    let content = "guardrail3 rs validate --staged .\n";
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
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_shebang_is_supported() {
    let content = "#!/usr/bin/env bash\nguardrail3 rs validate --staged .\n";
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
