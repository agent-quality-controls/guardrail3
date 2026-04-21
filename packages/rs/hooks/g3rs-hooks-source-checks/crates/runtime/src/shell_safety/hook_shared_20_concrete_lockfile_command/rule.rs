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
                "`.githooks/pre-commit` runs a concrete lockfile integrity command".to_owned(),
                "`.githooks/pre-commit` executes a real install verification command such as `pnpm install --frozen-lockfile`.".to_owned(),
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
        "missing concrete lockfile integrity command in `.githooks/pre-commit`".to_owned(),
        "Add an executable install verification step such as `pnpm install --frozen-lockfile` to `.githooks/pre-commit` when manifest or lockfile inputs change. Mentioning lockfiles in text or grepping filenames does not prove the lockfile still resolves.".to_owned(),
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
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
