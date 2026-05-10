#![expect(
    clippy::indexing_slicing,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use crate::inputs::ExecutableCommandContextInput;
use crate::support::{args_have_help_or_version, cargo_subcommand_tail};

/// `ID` constant.
const ID: &str = "g3rs-hooks/concrete-lockfile-command";

/// `check` function.
pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    if has_concrete_lockfile_command(input.parsed) {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Info,
                "`.githooks/pre-commit` runs a concrete lockfile integrity command".to_owned(),
                "`.githooks/pre-commit` executes a real Cargo lockfile verification command such as `cargo metadata --locked`.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
        return;
    }

    if delegates_to_g3rs_validate_staged(input.parsed) {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Info,
                "`.githooks/pre-commit` delegates lockfile integrity to g3rs validate --staged".to_owned(),
                "`.githooks/pre-commit` invokes `g3rs validate --path <unit> --staged`; lockfile integrity is enforced inside the in-binary validator.".to_owned(),
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
        "Add an executable Cargo lockfile verification step such as `cargo metadata --locked` to `.githooks/pre-commit`, or delegate to `g3rs validate --path <unit> --staged`. Mentioning lockfiles in text or grepping filenames does not prove the lockfile still resolves.".to_owned(),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

/// `has_concrete_lockfile_command` function.
fn has_concrete_lockfile_command(parsed: &hook_shell_parser::types::ParsedShellScript) -> bool {
    any_resolved_command(parsed, is_concrete_lockfile_command)
}

/// `is_concrete_lockfile_command` function.
fn is_concrete_lockfile_command(command: &ResolvedCommand) -> bool {
    cargo_subcommand_tail(command, "metadata").is_some_and(|args| {
        !args_have_help_or_version(args) && args.iter().any(|arg| arg == "--locked")
    })
}

/// `delegates_to_g3rs_validate_staged` function.
fn delegates_to_g3rs_validate_staged(parsed: &hook_shell_parser::types::ParsedShellScript) -> bool {
    any_resolved_command(parsed, |command| {
        if command.command_name() != "g3rs" {
            return false;
        }
        let args = command.args();
        if args.first().map(String::as_str) != Some("validate") {
            return false;
        }
        let tail = &args[1..];
        let has_staged = tail.iter().any(|arg| arg == "--staged");
        let has_path = tail
            .iter()
            .any(|arg| arg == "--path" || arg.starts_with("--path="));
        has_staged && has_path
    })
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
