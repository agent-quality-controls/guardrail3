use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "g3rs-hooks/merge-conflict-step-present";

pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    if has_merge_conflict_check(input.parsed) {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Info,
                "`.githooks/pre-commit` scans for merge-conflict markers".to_owned(),
                "`.githooks/pre-commit` includes an executable `rg` or `grep` step that scans for unresolved merge-conflict markers before commit.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
        return;
    }

    results.push(G3CheckResult::from_parts(
        ID.to_owned(),
        G3Severity::Warn,
        "missing merge-conflict marker scan in `.githooks/pre-commit`".to_owned(),
        "Add an executable `rg` or `grep` check for `<<<<<<<`, `=======`, and `>>>>>>>` near the start of `.githooks/pre-commit`, before expensive validation. This fails fast on unresolved merge conflicts.".to_owned(),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

fn has_merge_conflict_check(parsed: &hook_shell_parser::types::ParsedShellScript) -> bool {
    any_resolved_command(parsed, is_merge_conflict_command)
}

fn is_merge_conflict_command(command: &ResolvedCommand) -> bool {
    matches!(command.command_name(), "grep" | "rg")
        && (command.command_text().contains("<{7}")
            || command.command_text().contains("<<<<<<<")
            || command.command_text().contains("=======")
            || command.command_text().contains(">>>>>>>"))
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: crate::facts::HookScriptKind::PreCommit,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
