use guardrail3_app_rs_family_hooks_shared::hook_shell::ParsedShellScript;
use guardrail3_app_rs_family_hooks_shared::hook_shell::command_query::{
    ResolvedCommand, any_resolved_command,
};
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-08";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = script_contains_guardrail_step(input.parsed);

    push_presence_result(
        found,
        input.rel_path,
        "Rust guardrail validate step present",
        "Hook runs guardrail3 Rust validation on staged changes.",
        "Rust guardrail validate step missing",
        "Hook does not execute `guardrail3 ... validate --staged`.",
        results,
    );
}

pub(crate) fn script_contains_guardrail_step(parsed: &ParsedShellScript<'_>) -> bool {
    any_resolved_command(parsed, is_guardrail_validate_staged_command)
}

pub(crate) fn script_contains_path_qualified_guardrail_step(
    parsed: &ParsedShellScript<'_>,
) -> bool {
    any_resolved_command(parsed, is_path_qualified_guardrail_validate_staged_command)
}

fn is_path_qualified_guardrail_validate_staged_command(command: &ResolvedCommand) -> bool {
    command.path_qualified() && is_guardrail_validate_staged_command(command)
}

fn is_guardrail_validate_staged_command(command: &ResolvedCommand) -> bool {
    if command.command_name() != "guardrail3" {
        return false;
    }

    let args = command.args();
    let mut index = 0usize;

    while let Some(token) = args.get(index).map(String::as_str) {
        if !token.starts_with('-') {
            break;
        }

        if is_help_or_version_flag(token) {
            return false;
        }
        if let Some((flag_name, _)) = token.split_once('=')
            && guardrail_global_flag_takes_value(flag_name)
        {
            index += 1;
            continue;
        }
        if guardrail_global_flag_takes_value(token) {
            index += 2;
            continue;
        }

        index += 1;
    }

    let saw_validate = match args.get(index).map(String::as_str) {
        Some("rs") => args.get(index + 1).map(String::as_str) == Some("validate"),
        Some("validate") => true,
        _ => false,
    };

    if !saw_validate {
        return false;
    }

    args.iter()
        .skip(index)
        .all(|arg| !is_help_or_version_flag(arg))
        && args.iter().skip(index).any(|arg| arg == "--staged")
}

fn push_presence_result(
    found: bool,
    rel_path: &str,
    ok_title: &str,
    ok_message: &str,
    missing_title: &str,
    missing_message: &str,
    results: &mut Vec<CheckResult>,
) {
    if found {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                ok_title.to_owned(),
                ok_message.to_owned(),
                Some(rel_path.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            missing_title.to_owned(),
            missing_message.to_owned(),
            Some(rel_path.to_owned()),
            None,
            false,
        ));
    }
}

fn guardrail_global_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "--config" | "--format" | "--root" | "--cache-dir")
}

fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

#[cfg(test)]
pub(super) fn run_case(content: &str) -> Vec<CheckResult> {
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
#[path = "tests/steps/hook_rs_08_guardrail_validate_staged_present_tests/mod.rs"]
mod hook_rs_08_guardrail_validate_staged_present_tests;
