use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{
    CommandQueryOptions, CommandVisit, visit_resolved_commands_with_env,
};
use hook_shell_parser::types::ParsedShellScript;

use super::support::EnvState;
use crate::inputs::RustHookCommandInput;

const ID: &str = "RS-HOOKS-SOURCE-25";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Coverage {
    saw_cargo: bool,
    uncovered_cargo: bool,
}

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let coverage = script_coverage(input.parsed);
    if !coverage.saw_cargo {
        return;
    }

    if coverage.uncovered_cargo {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "missing shared `CARGO_TARGET_DIR` setup in `.githooks/pre-commit`".to_owned(),
            "Before the first cargo command in `.githooks/pre-commit`, set `REPO_ROOT=$(git rev-parse --show-toplevel)` and `export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"`. In a monorepo with multiple Cargo workspaces, this reuses one repo-local build cache instead of recompiling the same dependencies, proc-macros, and shared path dependencies in separate `target/` directories.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    } else {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "`.githooks/pre-commit` sets a shared `CARGO_TARGET_DIR`".to_owned(),
                "`.githooks/pre-commit` exports `CARGO_TARGET_DIR` before cargo runs, so Cargo workspaces reuse one repo-local build cache during the hook.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
    }
}

fn script_coverage(parsed: &ParsedShellScript) -> Coverage {
    let mut coverage = Coverage::default();
    visit_resolved_commands_with_env(
        parsed,
        EnvState::default(),
        CommandQueryOptions::default()
            .with_detached_commands()
            .with_forward_functions(),
        |command, state| {
            if command.command_name() == "cargo" {
                coverage.saw_cargo = true;
                coverage.uncovered_cargo |= !state.target_dir;
                if coverage.uncovered_cargo {
                    return CommandVisit::Stop;
                }
            }
            CommandVisit::Continue
        },
    );
    coverage
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        is_workspace_project: true,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
