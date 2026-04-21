use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command_relaxed};
use hook_shell_parser::types::ParsedShellScript;

use crate::inputs::RustHookCommandInput;

const ID: &str = "RS-HOOKS-SOURCE-14";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let found = script_contains_cargo_dupes_with_exclude_tests(input.parsed);

    if found {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Info,
                "`.githooks/pre-commit` runs `cargo dupes --exclude-tests`".to_owned(),
                "`.githooks/pre-commit` excludes test-only crates from the `cargo dupes` check."
                    .to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Info,
            "missing `--exclude-tests` on `cargo dupes` in `.githooks/pre-commit`"
                .to_owned(),
            "Change the `cargo dupes` command in `.githooks/pre-commit` to `cargo dupes --exclude-tests`. This keeps the duplication check focused on real workspace dependency versions instead of test-only crates.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

fn script_contains_cargo_dupes_with_exclude_tests(parsed: &ParsedShellScript) -> bool {
    any_resolved_command_relaxed(parsed, cargo_dupes_with_exclude_tests)
        && !any_resolved_command_relaxed(parsed, cargo_dupes_without_exclude_tests)
}

fn cargo_dupes_with_exclude_tests(command: &ResolvedCommand) -> bool {
    cargo_dupes_exclude_state(command) == Some(true)
}

fn cargo_dupes_without_exclude_tests(command: &ResolvedCommand) -> bool {
    cargo_dupes_exclude_state(command) == Some(false)
}

fn cargo_dupes_exclude_state(command: &ResolvedCommand) -> Option<bool> {
    match command.command_name() {
        "cargo-dupes" => cargo_dupes_binary_exclude_state(command.args()),
        "cargo" => cargo_dupes_subcommand_exclude_state(command.args()),
        _ => None,
    }
}

fn cargo_dupes_binary_exclude_state(args: &[String]) -> Option<bool> {
    let mut index = 0usize;
    let Some(subcommand) = args.get(index).map(String::as_str) else {
        return Some(false);
    };
    if subcommand.starts_with('-') || is_help_or_version_flag(subcommand) {
        return Some(false);
    }
    index += 1;

    dupes_flag_state(&args[index..])
}

fn cargo_dupes_subcommand_exclude_state(args: &[String]) -> Option<bool> {
    let mut index = 0usize;

    if args.get(index).is_some_and(|token| token.starts_with('+')) {
        index += 1;
    }

    while let Some(token) = args.get(index).map(String::as_str) {
        if !token.starts_with('-') {
            break;
        }

        if is_help_or_version_flag(token) {
            return None;
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

        return Some(false);
    }

    if args.get(index).map(String::as_str) != Some("dupes") {
        return None;
    }

    index += 1;
    let Some(subcommand) = args.get(index).map(String::as_str) else {
        return Some(false);
    };
    if subcommand.starts_with('-') || is_help_or_version_flag(subcommand) {
        return Some(false);
    }
    index += 1;

    dupes_flag_state(args.get(index..).unwrap_or(&[]))
}

fn dupes_flag_state(args: &[String]) -> Option<bool> {
    let mut index = 0usize;
    let mut exclude_tests = false;

    while let Some(token) = args.get(index).map(String::as_str) {
        if token == "--" {
            break;
        }
        if is_help_or_version_flag(token) {
            return Some(false);
        }
        if token == "--exclude-tests" {
            exclude_tests = true;
            index += 1;
            continue;
        }
        if let Some((flag_name, _)) = token.split_once('=')
            && dupes_flag_takes_value(flag_name)
        {
            index += 1;
            continue;
        }
        if dupes_flag_takes_value(token) {
            index += 2;
            continue;
        }
        if token.starts_with('-') {
            return Some(false);
        }
        index += 1;
    }

    Some(exclude_tests)
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

fn dupes_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "--max-exact" | "--max-exact-percent")
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
