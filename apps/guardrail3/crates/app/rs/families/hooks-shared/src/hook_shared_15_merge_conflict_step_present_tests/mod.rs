use crate::hook_shell::parse_script;
use guardrail3_domain_report::Severity;

use super::super::facts::HookScriptKind;
use super::super::hook_shared_15_merge_conflict_step_present::check;
use super::super::inputs::ExecutableCommandContextInput;

#[test]
fn warns_when_conflict_check_only_appears_in_comment() {
    let content = "# grep -qE '^(<{7} |={7}$|>{7} )' \"$file\"\n";
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
    assert_eq!(results[0].id, "HOOK-SHARED-15");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn passes_when_real_grep_conflict_command_exists() {
    let content = "grep -qE '^(<{7} |={7}$|>{7} )' \"$file\"\n";
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
fn warns_when_only_echo_mentions_conflict_markers() {
    let content = "echo \"Checking for merge conflict markers...\"\n";
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
    assert_eq!(results[0].id, "HOOK-SHARED-15");
    assert_eq!(results[0].severity, Severity::Warn);
}
