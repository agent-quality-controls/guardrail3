use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{
    CommandQueryOptions, CommandVisit, visit_resolved_commands_with_env,
};
use hook_shell_parser::types::ParsedShellScript;

use super::support::{EnvState, cargo_clippy_denies_warnings};
use crate::inputs::RustHookCommandInput;

/// `ID` constant.
const ID: &str = "g3rs-hooks/clippy-denies-warnings";

/// `check` function.
pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let found = script_contains_clippy_deny(input.parsed);

    if found {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Info,
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

/// `script_contains_clippy_deny` function.
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

/// Returns whether the script contains any executable cargo clippy command.
pub(crate) fn script_contains_clippy_command(parsed: &ParsedShellScript) -> bool {
    let mut found = false;
    visit_resolved_commands_with_env(
        parsed,
        EnvState::default(),
        CommandQueryOptions::default().with_forward_functions(),
        |command, _state| {
            if command.command_name() == "cargo"
                && crate::support::cargo_subcommand_tail(command, "clippy").is_some()
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
