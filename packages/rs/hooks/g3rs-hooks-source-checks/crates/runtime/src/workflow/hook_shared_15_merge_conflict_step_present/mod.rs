use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-15";

pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    if has_merge_conflict_check(input.parsed) {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Info,
                "merge-conflict check step present".to_owned(),
                "Hook contains a real executable merge-conflict marker check.".to_owned(),
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
        "merge-conflict check step missing".to_owned(),
        "Hook does not execute a merge-conflict marker scan before commit.".to_owned(),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

fn has_merge_conflict_check(parsed: &hook_shell_parser::ParsedShellScript<'_>) -> bool {
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
        content,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]

mod tests;
