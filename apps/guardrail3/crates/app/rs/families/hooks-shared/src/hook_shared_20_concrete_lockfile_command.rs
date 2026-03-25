use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-20";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    if has_concrete_lockfile_command(input.parsed.executable_lines.as_slice()) {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "concrete lockfile integrity command present".to_owned(),
                message: "Hook executes a real lockfile integrity command.".to_owned(),
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
        title: "concrete lockfile integrity command missing".to_owned(),
        message: "Hook mentions lockfiles without executing a concrete integrity command like `pnpm install --frozen-lockfile`.".to_owned(),
        file: Some(input.rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

fn has_concrete_lockfile_command(
    executable_lines: &[crate::hook_shell::ExecutableLine<'_>],
) -> bool {
    executable_lines.iter().any(|line| {
        line.command_name == "pnpm"
            && (line.command_text.contains("pnpm install") || line.command_text.contains("pnpm i"))
            && line.command_text.contains("--frozen-lockfile")
    })
}

#[cfg(test)]
#[path = "hook_shared_20_concrete_lockfile_command_tests/mod.rs"]
mod tests;
