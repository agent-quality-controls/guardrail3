use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-20";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    if has_concrete_lockfile_command(input.parsed.executable_lines.as_slice()) {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "concrete lockfile integrity command present".to_owned(),
                "Hook executes a real lockfile integrity command.".to_owned(),
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
    "concrete lockfile integrity command missing".to_owned(),
    "Hook mentions lockfiles without executing a concrete integrity command like `pnpm install --frozen-lockfile`.".to_owned(),
    Some(input.rel_path.to_owned()),
    None,
    false,
    ));
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
#[path = "hook_shared_20_concrete_lockfile_command_tests/mod.rs"]
mod hook_shared_20_concrete_lockfile_command_tests;
