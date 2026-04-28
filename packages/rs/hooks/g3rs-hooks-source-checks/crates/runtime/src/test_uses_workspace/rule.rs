use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};
use hook_shell_parser::types::ParsedShellScript;

use crate::inputs::RustHookCommandInput;

const ID: &str = "g3rs-hooks/test-uses-workspace";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    if !input.is_workspace_project {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Info,
                "cargo test workspace scope not required".to_owned(),
                "Hook is not attached to a Cargo workspace project, so `cargo test --workspace` is not required."
                    .to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
        return;
    }

    let found = script_contains_workspace_test(input.parsed);

    if found {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Info,
                "cargo test uses workspace scope".to_owned(),
                "Hook runs cargo test with `--workspace`.".to_owned(),
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
            "cargo test missing --workspace".to_owned(),
            "Hook does not execute `cargo test --workspace`.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

fn script_contains_workspace_test(parsed: &ParsedShellScript) -> bool {
    any_resolved_command(parsed, is_workspace_test_command)
}

fn is_workspace_test_command(command: &ResolvedCommand) -> bool {
    if command.command_name() != "cargo" {
        return false;
    }

    let args = command.args();
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

    if args.get(index).map(String::as_str) != Some("test") {
        return false;
    }

    let tail = args.get(index + 1..).unwrap_or(&[]);
    let relevant = tail
        .iter()
        .take_while(|arg| arg.as_str() != "--")
        .collect::<Vec<_>>();

    relevant.iter().all(|arg| !is_help_or_version_flag(arg))
        && relevant.iter().any(|arg| arg.as_str() == "--workspace")
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
    run_case_with_workspace(content, true)
}

#[cfg(test)]
pub(crate) fn run_case_with_workspace(
    content: &str,
    is_workspace_project: bool,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        is_workspace_project,
        requirements: &[],
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
