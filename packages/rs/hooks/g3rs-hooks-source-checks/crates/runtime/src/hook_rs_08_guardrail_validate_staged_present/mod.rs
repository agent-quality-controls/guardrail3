use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::types::ParsedShellScript;
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use super::inputs::RustHookCommandInput;

const ID: &str = "RS-HOOKS-SOURCE-09";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let found = script_contains_guardrail_step(input.parsed);

    push_presence_result(
        found,
        input.rel_path,
        "Rust guardrail validate step present",
        "Hook runs g3rs Rust validation with an explicit workspace path.",
        "Rust guardrail validate step missing",
        "Hook does not execute `g3rs validate --path ...`.",
        results,
    );
}

pub(crate) fn script_contains_guardrail_step(parsed: &ParsedShellScript) -> bool {
    any_resolved_command(parsed, is_guardrail_validate_path_command)
}

pub(crate) fn script_contains_path_qualified_guardrail_step(parsed: &ParsedShellScript) -> bool {
    any_resolved_command(parsed, is_path_qualified_guardrail_validate_path_command)
}

fn is_path_qualified_guardrail_validate_path_command(command: &ResolvedCommand) -> bool {
    command.path_qualified() && is_guardrail_validate_path_command(command)
}

fn is_guardrail_validate_path_command(command: &ResolvedCommand) -> bool {
    if command.command_name() != "g3rs" {
        return false;
    }

    let args = command.args();
    if args
        .first()
        .is_some_and(|token| token.starts_with('-') || is_help_or_version_flag(token))
    {
        return false;
    }

    if args.first().map(String::as_str) != Some("validate") {
        return false;
    }

    parse_validate_args(&args[1..])
}

fn parse_validate_args(args: &[String]) -> bool {
    let mut saw_path = false;
    let mut index = 0usize;
    while let Some(arg) = args.get(index).map(String::as_str) {
        if is_help_or_version_flag(arg) {
            return false;
        }
        if let Some(path) = arg.strip_prefix("--path=") {
            if path.is_empty() {
                return false;
            }
            saw_path = true;
            index += 1;
            continue;
        }
        if arg == "--path" {
            let Some(value) = args.get(index + 1).map(String::as_str) else {
                return false;
            };
            if value.starts_with('-') {
                return false;
            }
            saw_path = true;
            index += 2;
            continue;
        }
        if let Some(value) = arg.strip_prefix("--family=") {
            if value.is_empty() {
                return false;
            }
            index += 1;
            continue;
        }
        if arg == "--family" {
            let Some(value) = args.get(index + 1).map(String::as_str) else {
                return false;
            };
            if value.starts_with('-') {
                return false;
            }
            index += 2;
            continue;
        }
        if arg == "--inventory" {
            index += 1;
            continue;
        }
        return false;
    }
    saw_path
}

fn push_presence_result(
    found: bool,
    rel_path: &str,
    ok_title: &str,
    ok_message: &str,
    missing_title: &str,
    missing_message: &str,
    results: &mut Vec<G3CheckResult>,
) {
    if found {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Info,
                ok_title.to_owned(),
                ok_message.to_owned(),
                Some(rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            missing_title.to_owned(),
            missing_message.to_owned(),
            Some(rel_path.to_owned()),
            None,
            false,
        ));
    }
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
mod tests;
