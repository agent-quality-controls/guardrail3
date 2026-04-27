use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};
use hook_shell_parser::types::ParsedShellScript;

use crate::inputs::RustHookCommandInput;

const ID: &str = "g3rs-hooks/cargo-dupes-step-present";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let found = script_contains_cargo_dupes(input.parsed);

    if found {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "`.githooks/pre-commit` runs `cargo dupes`".to_owned(),
                "`.githooks/pre-commit` includes an executable `cargo dupes` command in the Rust check flow.".to_owned(),
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
            "missing executable `cargo dupes` command in `.githooks/pre-commit`".to_owned(),
            "Add `cargo dupes --exclude-tests` as a real command in `.githooks/pre-commit`, with the other Rust cargo checks. This hook rule is about duplicate Rust dependency versions inside each Cargo workspace, not copied source text.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

pub(crate) fn script_contains_cargo_dupes(parsed: &ParsedShellScript) -> bool {
    any_resolved_command(parsed, is_cargo_dupes_command)
}

fn is_cargo_dupes_command(command: &ResolvedCommand) -> bool {
    match command.command_name() {
        "cargo-dupes" => !command
            .args()
            .iter()
            .any(|arg| is_help_or_version_flag(arg)),
        "cargo" => cargo_dupes_subcommand_invocation(command.args()),
        _ => false,
    }
}

fn cargo_dupes_subcommand_invocation(args: &[String]) -> bool {
    let mut index = 0usize;

    if args.get(index).is_some_and(|token| token.starts_with('+')) {
        index += 1;
    }

    while let Some(token) = args.get(index).map(String::as_str) {
        if !token.starts_with('-') {
            break;
        }

        if is_help_or_version_flag(token) {
            return false;
        }
        if let Some((flag_name, _)) = token.split_once('=')
            && cargo_global_flag_takes_value(flag_name)
        {
            index += 1;
            continue;
        }
        if matches!(token.strip_prefix("-j"), Some(value) if !value.is_empty()) {
            index += 1;
            continue;
        }
        if cargo_global_flag_takes_value(token) {
            index += 2;
            continue;
        }

        index += 1;
    }

    if args.get(index).map(String::as_str) != Some("dupes") {
        return false;
    }

    !args
        .get(index + 1..)
        .unwrap_or(&[])
        .iter()
        .any(|arg| is_help_or_version_flag(arg))
}

fn cargo_global_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "--config"
            | "-Z"
            | "--manifest-path"
            | "--color"
            | "--target"
            | "--target-dir"
            | "--jobs"
            | "-j"
            | "-C"
    )
}

fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
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
