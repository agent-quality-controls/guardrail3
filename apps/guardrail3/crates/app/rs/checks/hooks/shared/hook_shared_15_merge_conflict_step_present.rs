use crate::domain::report::{CheckResult, Severity};

use super::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-15";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    if has_merge_conflict_check(input.parsed.executable_lines.as_slice()) {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "merge-conflict check step present".to_owned(),
                message: "Hook contains a real executable merge-conflict marker check.".to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "merge-conflict check step missing".to_owned(),
        message: "Hook does not execute a merge-conflict marker scan before commit.".to_owned(),
        file: Some(input.rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

fn has_merge_conflict_check(
    executable_lines: &[crate::app::rs::checks::hooks::shell::ExecutableLine<'_>],
) -> bool {
    executable_lines.iter().any(|line| {
        let command = line.command_text.trim();
        let command_name = command.split_whitespace().next().unwrap_or_default();
        matches!(command_name, "grep" | "rg")
            && (command.contains("<{7}")
                || command.contains("<<<<<<<")
                || command.contains("=======")
                || command.contains(">>>>>>>")
                || command.contains("conflict marker")
                || command.contains("merge conflict"))
    })
}

#[cfg(test)]
#[path = "hook_shared_15_merge_conflict_step_present_tests/mod.rs"]
mod tests;
