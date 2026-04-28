use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{
    CommandQueryOptions, CommandVisit, visit_resolved_commands_with_env,
};
use hook_shell_parser::types::ParsedShellScript;

use super::support::{EnvState, cargo_clippy_denies_warnings};
use crate::inputs::RustHookCommandInput;

const ID: &str = "g3rs-hooks/clippy-denies-warnings";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let found = script_contains_clippy_deny(input.parsed);

    if found {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "`.githooks/pre-commit` runs clippy in deny-warnings mode".to_owned(),
                "`.githooks/pre-commit` already executes `cargo clippy` with `-D warnings` or an equivalent `RUSTFLAGS` deny setting, so any clippy warning fails the hook.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "missing deny-warnings `cargo clippy` command in `.githooks/pre-commit`"
                .to_owned(),
            "Add a real clippy command such as `cargo clippy --workspace --all-targets --all-features -- -D warnings` to `.githooks/pre-commit`. Put it with the other Rust cargo checks so the hook fails on any clippy warning, not only on hard errors.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

pub(crate) fn script_contains_clippy_deny(parsed: &ParsedShellScript) -> bool {
    let mut found = false;
    visit_resolved_commands_with_env(
        parsed,
        EnvState::default(),
        CommandQueryOptions::default().with_forward_functions(),
        |command, state| {
            if command.command_name() == "cargo"
                && cargo_clippy_denies_warnings(command.args(), state)
            {
                found = true;
                CommandVisit::Stop
            } else {
                CommandVisit::Continue
            }
        },
    );
    found
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        is_workspace_project: true,
        requirements: &[],
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
