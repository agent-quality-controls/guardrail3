use crate::hook_shell::parse_script;
use guardrail3_domain_report::Severity;

use super::super::facts::HookScriptKind;
use super::super::hook_shared_20_concrete_lockfile_command::check;
use super::super::inputs::ExecutableCommandContextInput;

#[test]
fn warns_when_lockfile_check_is_only_prose() {
    let content = "echo \"run pnpm install --frozen-lockfile\"\n";
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
    assert_eq!(results[0].id, "HOOK-SHARED-20");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn passes_when_real_frozen_lockfile_command_exists() {
    let content = "pnpm install --frozen-lockfile\n";
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
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_frozen_lockfile_command_is_echoed() {
    let content = "echo \"pnpm install --frozen-lockfile\"\n";
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
