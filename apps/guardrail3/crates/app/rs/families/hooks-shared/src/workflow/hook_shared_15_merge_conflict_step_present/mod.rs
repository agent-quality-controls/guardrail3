use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-15";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    if has_merge_conflict_check(input.parsed.executable_lines.as_slice()) {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "merge-conflict check step present".to_owned(),
                "Hook contains a real executable merge-conflict marker check.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Warn,
        "merge-conflict check step missing".to_owned(),
        "Hook does not execute a merge-conflict marker scan before commit.".to_owned(),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

fn has_merge_conflict_check(executable_lines: &[crate::hook_shell::ExecutableLine<'_>]) -> bool {
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
pub(crate) fn run_case(content: &str) -> Vec<CheckResult> {
    let parsed = crate::hook_shell::parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: crate::facts::HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]

mod hook_shared_15_merge_conflict_step_present_tests;
