use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{
    ResolvedCommand, any_resolved_command, any_resolved_command_on_line,
};
use hook_shell_parser::types::ParsedShellScript;

use crate::inputs::RustHookCommandInput;

const ID: &str = "g3rs-hooks/guardrail-validate-staged-present";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let found = script_contains_guardrail_step(input.parsed);

    push_presence_result(
        found,
        input.rel_path,
        "`.githooks/pre-commit` runs `g3rs validate --path ...`",
        "`.githooks/pre-commit` includes an executable `g3rs validate --path ...` command, so Rust guardrail rules run during the hook.",
        "missing `g3rs validate --path ...` command in `.githooks/pre-commit`",
        "Add an executable `g3rs validate --path \"$rust_root\"` step to `.githooks/pre-commit`, next to the Rust cargo checks. Cargo tools do not cover guardrail configuration and architecture rules.",
        results,
    );
}

pub(crate) fn script_contains_guardrail_step(parsed: &ParsedShellScript) -> bool {
    any_resolved_command(parsed, is_guardrail_validate_path_command)
}

pub(crate) fn line_contains_guardrail_step(
    parsed: &ParsedShellScript,
    raw: &str,
    line_no: usize,
) -> bool {
    any_resolved_command_on_line(parsed, raw, line_no, &is_guardrail_validate_path_command)
}

pub(crate) fn line_contains_path_qualified_guardrail_step(
    parsed: &ParsedShellScript,
    raw: &str,
    line_no: usize,
) -> bool {
    any_resolved_command_on_line(
        parsed,
        raw,
        line_no,
        &is_path_qualified_guardrail_validate_path_command,
    )
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
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
