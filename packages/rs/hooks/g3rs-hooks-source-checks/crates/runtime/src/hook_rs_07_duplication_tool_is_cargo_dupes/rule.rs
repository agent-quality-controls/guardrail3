use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use crate::inputs::RustHookCommandInput;

const ID: &str = "RS-HOOKS-SOURCE-08";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let has_cargo_dupes = any_resolved_command(input.parsed, is_cargo_dupes_command);
    let has_jscpd = any_resolved_command(input.parsed, is_jscpd_command);

    if has_jscpd && !has_cargo_dupes {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "replace `jscpd` with `cargo dupes --exclude-tests` for Rust dependency duplication"
                .to_owned(),
            "`.githooks/pre-commit` runs `jscpd`, but `jscpd` checks copied text, not duplicate Rust dependency versions. Add a real `cargo dupes --exclude-tests` command to the Rust cargo-check section of `.githooks/pre-commit`.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    } else if has_cargo_dupes {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "`.githooks/pre-commit` uses `cargo dupes` for Rust dependency duplication"
                    .to_owned(),
                "`.githooks/pre-commit` includes an executable `cargo dupes` command for Rust dependency-duplication checks.".to_owned(),
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
            "missing `cargo dupes --exclude-tests` command in `.githooks/pre-commit`"
                .to_owned(),
            "Add an executable `cargo dupes --exclude-tests` command to `.githooks/pre-commit`, next to the other Rust cargo checks. This checks duplicate Rust dependency versions inside each Cargo workspace; repo-wide text duplication tools do not cover that.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
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

    args.get(index).map(String::as_str) == Some("dupes")
        && !args
            .get(index + 1..)
            .unwrap_or(&[])
            .iter()
            .any(|arg| is_help_or_version_flag(arg))
}

fn is_jscpd_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "jscpd"
        && !command
            .args()
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
