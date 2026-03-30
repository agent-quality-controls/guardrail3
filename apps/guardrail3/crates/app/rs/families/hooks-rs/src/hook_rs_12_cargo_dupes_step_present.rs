mod support;

use guardrail3_app_rs_family_hooks_shared::hook_shell::{ParsedShellScript, parse_script};
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;
use self::support::*;

const ID: &str = "HOOK-RS-12";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = script_contains_cargo_dupes(input.parsed);

    if found {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "cargo-dupes step present".to_owned(),
                message: "Hook runs cargo-dupes as an executable command.".to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "cargo-dupes step missing".to_owned(),
            message: "Hook does not execute cargo-dupes.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

pub(crate) fn script_contains_cargo_dupes(parsed: &ParsedShellScript<'_>) -> bool {
    parsed
        .executable_lines
        .iter()
        .any(|line| line_contains_cargo_dupes(line.raw, parsed, &mut Vec::new()))
}

pub(crate) fn script_contains_path_qualified_cargo_dupes(parsed: &ParsedShellScript<'_>) -> bool {
    parsed
        .executable_lines
        .iter()
        .any(|line| line_contains_path_qualified_cargo_dupes(line.raw, parsed, &mut Vec::new()))
}

fn line_contains_cargo_dupes(
    raw: &str,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
) -> bool {
    let segments = split_command_segments(raw);
    if segments.is_empty() {
        return segment_contains_cargo_dupes(raw, root, visiting);
    }

    let mut prefix_status = None;
    for segment in segments {
        let reachable = match (segment.operator_before, prefix_status) {
            (Some("&&"), Some(true)) => true,
            (Some("&&"), Some(false)) => false,
            (Some("||"), Some(true)) => false,
            (Some("||"), Some(false)) => true,
            _ => true,
        };

        if reachable && segment.operator_after != Some("&") && segment.operator_after != Some("|") {
            if segment_contains_cargo_dupes(&segment.text, root, visiting) {
                return true;
            }
            for substitution in extract_command_substitutions(&segment.text) {
                if line_contains_cargo_dupes(&substitution, root, visiting) {
                    return true;
                }
            }
        }

        if reachable {
            prefix_status = constant_exit_status(&segment.text);
        }
    }

    false
}

fn line_contains_path_qualified_cargo_dupes(
    raw: &str,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
) -> bool {
    let segments = split_command_segments(raw);
    if segments.is_empty() {
        return segment_contains_path_qualified_cargo_dupes(raw, root, visiting);
    }

    let mut prefix_status = None;
    for segment in segments {
        let reachable = match (segment.operator_before, prefix_status) {
            (Some("&&"), Some(true)) => true,
            (Some("&&"), Some(false)) => false,
            (Some("||"), Some(true)) => false,
            (Some("||"), Some(false)) => true,
            _ => true,
        };

        if reachable && segment.operator_after != Some("&") && segment.operator_after != Some("|") {
            if segment_contains_path_qualified_cargo_dupes(&segment.text, root, visiting) {
                return true;
            }
            for substitution in extract_command_substitutions(&segment.text) {
                if line_contains_path_qualified_cargo_dupes(&substitution, root, visiting) {
                    return true;
                }
            }
        }

        if reachable {
            prefix_status = constant_exit_status(&segment.text);
        }
    }

    false
}

fn segment_contains_cargo_dupes(
    segment: &str,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
) -> bool {
    let tokens = shell_words(segment);
    let mut parts = tokens.iter().map(String::as_str).peekable();

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    let Some(first) = parts.next() else {
        return false;
    };

    match normalize_command_token(first) {
        "env" => env_wrapper_contains_cargo_dupes(parts, root, visiting),
        "sh" | "bash" => shell_wrapper_contains_cargo_dupes(parts, root, visiting),
        "command" => command_wrapper_contains_cargo_dupes(parts, root, visiting),
        "exec" => exec_wrapper_contains_cargo_dupes(parts, root, visiting),
        "cargo" => cargo_dupes_subcommand_invocation(&mut parts),
        "cargo-dupes" => cargo_dupes_binary_invocation(&mut parts),
        command_name => called_function_contains_cargo_dupes(command_name, root, visiting),
    }
}

fn segment_contains_path_qualified_cargo_dupes(
    segment: &str,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
) -> bool {
    let tokens = shell_words(segment);
    let mut parts = tokens.iter().map(String::as_str).peekable();

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    let Some(first) = parts.next() else {
        return false;
    };

    match normalize_command_token(first) {
        "env" => env_wrapper_contains_path_qualified_cargo_dupes(parts, root, visiting),
        "sh" | "bash" => shell_wrapper_contains_path_qualified_cargo_dupes(parts, root, visiting),
        "command" => command_wrapper_contains_path_qualified_cargo_dupes(parts, root, visiting),
        "exec" => exec_wrapper_contains_path_qualified_cargo_dupes(parts, root, visiting),
        "cargo" => first.contains('/') && cargo_dupes_subcommand_invocation(&mut parts),
        "cargo-dupes" => first.contains('/') && cargo_dupes_binary_invocation(&mut parts),
        command_name => {
            called_function_contains_path_qualified_cargo_dupes(command_name, root, visiting)
        }
    }
}

fn called_function_contains_cargo_dupes(
    command_name: &str,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
) -> bool {
    let Some(function) = root
        .functions
        .iter()
        .find(|function| function.name == command_name)
    else {
        return false;
    };
    if visiting.iter().any(|name| name == &function.name) {
        return false;
    }

    visiting.push(function.name.clone());
    let body_parsed = parse_script(&function.body);
    let found = body_parsed
        .executable_lines
        .iter()
        .any(|line| line_contains_cargo_dupes(line.raw, root, visiting));
    let _ = visiting.pop();
    found
}

fn called_function_contains_path_qualified_cargo_dupes(
    command_name: &str,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
) -> bool {
    let Some(function) = root
        .functions
        .iter()
        .find(|function| function.name == command_name)
    else {
        return false;
    };
    if visiting.iter().any(|name| name == &function.name) {
        return false;
    }

    visiting.push(function.name.clone());
    let body_parsed = parse_script(&function.body);
    let found = body_parsed
        .executable_lines
        .iter()
        .any(|line| line_contains_path_qualified_cargo_dupes(line.raw, root, visiting));
    let _ = visiting.pop();
    found
}

fn env_wrapper_contains_cargo_dupes<'a, I>(
    mut parts: std::iter::Peekable<I>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let mut split_string = None;

    while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if flag == "--" {
            break;
        }
        if let Some((flag_name, value)) = flag.split_once('=')
            && env_flag_takes_value(flag_name)
        {
            if matches!(flag_name, "-S" | "--split-string") {
                split_string = Some(value.to_owned());
            }
            continue;
        }
        if env_flag_takes_value(flag) {
            let value = parts.next().unwrap_or_default();
            if matches!(flag, "-S" | "--split-string") {
                split_string = Some(value.to_owned());
            }
        }
    }

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    if let Some(script) = split_string {
        let mut nested = script;
        let tail: Vec<_> = parts.map(str::to_owned).collect();
        if !tail.is_empty() {
            nested.push(' ');
            nested.push_str(&tail.join(" "));
        }
        return line_contains_cargo_dupes(&nested, root, visiting);
    }

    let Some(next) = parts.next() else {
        return false;
    };

    wrapper_or_command_contains_cargo_dupes(next, &mut parts, root, visiting)
}

fn env_wrapper_contains_path_qualified_cargo_dupes<'a, I>(
    mut parts: std::iter::Peekable<I>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let mut split_string = None;

    while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if flag == "--" {
            break;
        }
        if let Some((flag_name, value)) = flag.split_once('=')
            && env_flag_takes_value(flag_name)
        {
            if matches!(flag_name, "-S" | "--split-string") {
                split_string = Some(value.to_owned());
            }
            continue;
        }
        if env_flag_takes_value(flag) {
            let value = parts.next().unwrap_or_default();
            if matches!(flag, "-S" | "--split-string") {
                split_string = Some(value.to_owned());
            }
        }
    }

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    if let Some(script) = split_string {
        let mut nested = script;
        let tail: Vec<_> = parts.map(str::to_owned).collect();
        if !tail.is_empty() {
            nested.push(' ');
            nested.push_str(&tail.join(" "));
        }
        return line_contains_path_qualified_cargo_dupes(&nested, root, visiting);
    }

    let Some(next) = parts.next() else {
        return false;
    };

    wrapper_or_command_contains_path_qualified_cargo_dupes(next, &mut parts, root, visiting)
}

fn shell_wrapper_contains_cargo_dupes<'a, I>(
    parts: std::iter::Peekable<I>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let mut parts = parts;
    while let Some(token) = parts.peek().copied() {
        if !token.starts_with('-') {
            break;
        }
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if let Some((flag_name, _)) = flag.split_once('=')
            && shell_flag_takes_value(flag_name)
        {
            continue;
        }
        if shell_flag_takes_value(flag) {
            let _ = parts.next();
        }
    }

    let Some(script) = parts.next() else {
        return false;
    };

    line_contains_cargo_dupes(script, root, visiting)
}

fn shell_wrapper_contains_path_qualified_cargo_dupes<'a, I>(
    parts: std::iter::Peekable<I>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let mut parts = parts;
    while let Some(token) = parts.peek().copied() {
        if !token.starts_with('-') {
            break;
        }
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if let Some((flag_name, _)) = flag.split_once('=')
            && shell_flag_takes_value(flag_name)
        {
            continue;
        }
        if shell_flag_takes_value(flag) {
            let _ = parts.next();
        }
    }

    let Some(script) = parts.next() else {
        return false;
    };

    line_contains_path_qualified_cargo_dupes(script, root, visiting)
}

fn command_wrapper_contains_cargo_dupes<'a, I>(
    parts: std::iter::Peekable<I>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let mut parts = parts;
    while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) || matches!(flag, "-v" | "-V") {
            return false;
        }
        if flag == "--" {
            break;
        }
        if flag != "-p" {
            return false;
        }
    }

    let Some(next) = parts.next() else {
        return false;
    };

    wrapper_or_command_contains_cargo_dupes(next, &mut parts, root, visiting)
}

fn command_wrapper_contains_path_qualified_cargo_dupes<'a, I>(
    parts: std::iter::Peekable<I>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let mut parts = parts;
    while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) || matches!(flag, "-v" | "-V") {
            return false;
        }
        if flag == "--" {
            break;
        }
        if flag != "-p" {
            return false;
        }
    }

    let Some(next) = parts.next() else {
        return false;
    };

    wrapper_or_command_contains_path_qualified_cargo_dupes(next, &mut parts, root, visiting)
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
#[path = "hook_rs_12_cargo_dupes_step_present_tests/mod.rs"]
mod hook_rs_12_cargo_dupes_step_present_tests;
