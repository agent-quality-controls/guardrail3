use guardrail3_app_rs_family_hooks_shared::hook_shell::ParsedShellScript;
use guardrail3_app_rs_family_hooks_shared::hook_shell::command_query::{
    ResolvedCommand, any_resolved_command,
};
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-10";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = script_contains_workspace_test(input.parsed);

    if found {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "cargo test uses workspace scope".to_owned(),
                "Hook runs cargo test with `--workspace`.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "cargo test missing --workspace".to_owned(),
            "Hook does not execute `cargo test --workspace`.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

fn script_contains_workspace_test(parsed: &ParsedShellScript<'_>) -> bool {
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
pub(crate) fn run_case(content: &str) -> Vec<CheckResult> {
    let parsed = test_support::parsed_hook(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]

mod tests;
