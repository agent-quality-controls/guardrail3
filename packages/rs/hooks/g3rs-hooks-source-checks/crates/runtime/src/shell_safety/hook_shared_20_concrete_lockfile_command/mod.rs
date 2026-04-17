use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "RS-HOOKS-SOURCE-23";

pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    if has_concrete_lockfile_command(input.parsed) {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Info,
                "concrete lockfile integrity command present".to_owned(),
                "Hook executes a real lockfile integrity command.".to_owned(),
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
    "concrete lockfile integrity command missing".to_owned(),
    "Hook mentions lockfiles without executing a concrete integrity command like `pnpm install --frozen-lockfile`.".to_owned(),
    Some(input.rel_path.to_owned()),
    None,
    false,
    ));
}

fn has_concrete_lockfile_command(parsed: &hook_shell_parser::types::ParsedShellScript) -> bool {
    any_resolved_command(parsed, is_concrete_lockfile_command)
}

fn is_concrete_lockfile_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "pnpm"
        && matches!(
            command.args().first().map(String::as_str),
            Some("install" | "i")
        )
        && command.args().iter().any(|arg| arg == "--frozen-lockfile")
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
mod tests;
