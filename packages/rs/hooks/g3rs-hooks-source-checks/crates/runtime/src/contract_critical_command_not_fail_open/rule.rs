#![expect(
    clippy::arithmetic_side_effects,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::string_slice,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::too_many_arguments,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::type_complexity,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
use std::collections::BTreeSet;

use crate::compat::{G3CheckResult, G3Severity};
use crate::inputs::FailOpenWrapperInput;
use g3rs_hooks_contract_types::G3HookCriticalCommand;
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command_on_line_in_context};
use hook_shell_parser::types::{ParsedShellScript, ShellFunction};

/// `ID` constant.
const ID: &str = "g3rs-hooks/contract-critical-command-not-fail-open";

/// `check` function.
pub(crate) fn check(input: &FailOpenWrapperInput<'_>, results: &mut Vec<G3CheckResult>) {
    let critical = critical_commands(input);
    if critical.is_empty() {
        return;
    }

    if let Some((line_no, command_text)) = first_fail_open_critical_command(
        input.parsed,
        input.parsed,
        1,
        0,
        &critical,
        &mut Vec::new(),
    ) {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Error,
            "contract-critical hook command is fail-open".to_owned(),
            format!(
                "Critical hook command `{command_text}` is softened by a fail-open wrapper. Critical commands come from family hook contracts; do not hide failures with `|| true`, `|| echo ...`, or equivalent wrappers."
            ),
            Some(input.rel_path.to_owned()),
            Some(line_no),
            false,
        ));
    }
}

/// `critical_commands` function.
fn critical_commands(input: &FailOpenWrapperInput<'_>) -> Vec<G3HookCriticalCommand> {
    let mut commands = BTreeSet::from([
        G3HookCriticalCommand::Binary("g3rs".to_owned()),
        G3HookCriticalCommand::Binary("gitleaks".to_owned()),
    ]);
    for requirement in input.requirements {
        commands.extend(requirement.critical_commands.iter().cloned());
    }
    commands.into_iter().collect()
}

/// `first_fail_open_critical_command` function.
fn first_fail_open_critical_command(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    absolute_base: usize,
    root_line_no: usize,
    critical: &[G3HookCriticalCommand],
    visiting: &mut Vec<String>,
) -> Option<(usize, String)> {
    for line in &local.executable_lines {
        let absolute_line_no = absolute_base + line.line_no.saturating_sub(1);
        let visible_root_line_no = if std::ptr::eq(local, root) {
            absolute_line_no
        } else {
            root_line_no
        };

        if line.softened_by.is_some()
            && any_resolved_command_on_line_in_context(
                local,
                root,
                &line.raw,
                line.line_no,
                visible_root_line_no,
                |command| command_matches_critical(command, critical),
            )
        {
            return Some((absolute_line_no, line.command_text.clone()));
        }
        if negated_if_failure_branch_is_softened(local, line.line_no)
            && any_resolved_command_on_line_in_context(
                local,
                root,
                &line.raw,
                line.line_no,
                visible_root_line_no,
                |command| command_matches_critical(command, critical),
            )
        {
            return Some((absolute_line_no, line.command_text.clone()));
        }
        if positive_availability_guard_is_softened(local, line.line_no, critical) {
            return Some((absolute_line_no, line.command_text.clone()));
        }
        if let Some(found) = called_function_fail_open(
            local,
            root,
            &line.command_name,
            line.line_no,
            absolute_base,
            visible_root_line_no,
            critical,
            visiting,
        ) {
            return Some(found);
        }
    }

    None
}

/// `positive_availability_guard_is_softened` function.
fn positive_availability_guard_is_softened(
    script: &ParsedShellScript,
    line_no: usize,
    critical: &[G3HookCriticalCommand],
) -> bool {
    let Some(tool) = positive_availability_guard_tool(script, line_no) else {
        return false;
    };
    if !tool_matches_critical(tool.as_str(), critical) {
        return false;
    }
    let else_branch = positive_if_else_branch(script, line_no).unwrap_or_default();
    !branch_has_failure_terminator(&else_branch, script, line_no)
}

/// `positive_availability_guard_tool` function.
fn positive_availability_guard_tool(script: &ParsedShellScript, line_no: usize) -> Option<String> {
    let raw = script
        .source_lines
        .iter()
        .find(|line| line.line_no == line_no)?
        .raw
        .as_str();
    let trimmed = raw.trim_start();
    if !trimmed.starts_with("if ") || !trimmed.contains("then") {
        return None;
    }
    let after_if = trimmed.strip_prefix("if ")?.trim_start();
    // A negated `if ! ...; then exit 1; fi` is the failure-handler shape, not a positive
    // availability guard. The negated form is analysed by `negated_if_failure_branch_is_softened`.
    if after_if.starts_with('!') {
        return None;
    }
    let words = hook_shell_parser::command_query::shell_words(after_if);
    if words.first().is_some_and(|word| word == "command")
        && words.get(1).is_some_and(|word| word == "-v")
    {
        return words.get(2).cloned();
    }
    None
}

/// `positive_if_else_branch` function.
fn positive_if_else_branch(script: &ParsedShellScript, line_no: usize) -> Option<String> {
    let mut found_then = false;
    let mut found_else = false;
    let mut branch = String::new();
    for line in script
        .source_lines
        .iter()
        .filter(|line| line.line_no >= line_no)
    {
        let mut text = line.raw.as_str();
        if !found_then {
            let Some((_, after_then)) = text.split_once("then") else {
                continue;
            };
            found_then = true;
            text = after_then;
        }
        if !found_else {
            if let Some((_, after_else)) = split_control_token(text, "else") {
                found_else = true;
                text = after_else;
            } else if split_control_token(text, "fi").is_some() {
                return None;
            } else {
                continue;
            }
        }
        if let Some((before_fi, _)) = split_control_token(text, "fi") {
            branch.push_str(before_fi);
            return Some(branch);
        }
        branch.push_str(text);
        branch.push('\n');
    }
    found_else.then_some(branch)
}

/// `tool_matches_critical` function.
fn tool_matches_critical(tool: &str, critical: &[G3HookCriticalCommand]) -> bool {
    critical.iter().any(|expected| match expected {
        G3HookCriticalCommand::Binary(binary) => binary == tool,
        G3HookCriticalCommand::CargoSubcommand(_) => tool == "cargo",
    })
}

/// `negated_if_failure_branch_is_softened` function.
fn negated_if_failure_branch_is_softened(script: &ParsedShellScript, line_no: usize) -> bool {
    let Some(branch) = negated_if_failure_branch(script, line_no) else {
        return false;
    };
    !branch_has_failure_terminator(&branch, script, line_no)
}

/// `negated_if_failure_branch` function.
fn negated_if_failure_branch(script: &ParsedShellScript, line_no: usize) -> Option<String> {
    let start = script
        .source_lines
        .iter()
        .find(|line| line.line_no == line_no)?;
    if !starts_negated_if(start.raw.as_str()) {
        return None;
    }

    let mut branch = String::new();
    let mut found_then = false;
    for line in script
        .source_lines
        .iter()
        .filter(|line| line.line_no >= line_no)
    {
        let mut text = line.raw.as_str();
        if !found_then {
            let Some((_, after_then)) = text.split_once("then") else {
                continue;
            };
            found_then = true;
            text = after_then;
        }

        if let Some((before_else, _)) = split_control_token(text, "else") {
            branch.push_str(before_else);
            break;
        }
        if let Some((before_elif, _)) = split_control_token(text, "elif") {
            branch.push_str(before_elif);
            break;
        }
        if let Some((before_fi, _)) = split_control_token(text, "fi") {
            branch.push_str(before_fi);
            break;
        }

        branch.push_str(text);
        branch.push('\n');
    }

    found_then.then_some(branch)
}

/// `starts_negated_if` function.
fn starts_negated_if(raw: &str) -> bool {
    let trimmed = raw.trim_start();
    trimmed
        .strip_prefix("if")
        .is_some_and(|tail| tail.trim_start().starts_with('!'))
}

/// `split_control_token` function.
fn split_control_token<'a>(text: &'a str, token: &str) -> Option<(&'a str, &'a str)> {
    for (index, _) in text.match_indices(token) {
        let before = &text[..index];
        let after = &text[index + token.len()..];
        let before_ok = before
            .chars()
            .last()
            .is_none_or(|ch| ch.is_whitespace() || ch == ';');
        let after_ok = after
            .chars()
            .next()
            .is_none_or(|ch| ch.is_whitespace() || ch == ';');
        if before_ok && after_ok {
            return Some((before, after));
        }
    }
    None
}

/// `branch_has_failure_terminator` function.
fn branch_has_failure_terminator(
    branch: &str,
    root: &ParsedShellScript,
    visible_root_line_no: usize,
) -> bool {
    let parsed = hook_shell_parser::parse_script(branch);
    parsed.executable_lines.iter().any(|line| {
        matches!(line.command_name.as_str(), "false")
            || (matches!(line.command_name.as_str(), "exit" | "return")
                && command_second_word_is_nonzero(line.command_text.as_str()))
            || called_function_terminates_failure(root, &line.command_name, visible_root_line_no)
    })
}

/// `called_function_terminates_failure` function.
fn called_function_terminates_failure(
    parsed: &ParsedShellScript,
    command_name: &str,
    line_no: usize,
) -> bool {
    parsed
        .functions
        .iter()
        .rev()
        .find(|function| function.name == command_name && function.line_no <= line_no)
        .is_some_and(|function| {
            branch_has_failure_terminator(&function.body, parsed, function.line_no)
        })
}

/// `command_second_word_is_nonzero` function.
fn command_second_word_is_nonzero(command_text: &str) -> bool {
    command_text
        .split_whitespace()
        .nth(1)
        .and_then(|value| value.trim_end_matches(';').parse::<u8>().ok())
        .is_some_and(|value| value != 0)
}

/// `called_function_fail_open` function.
fn called_function_fail_open(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    command_name: &str,
    call_line_no: usize,
    absolute_base: usize,
    root_line_no: usize,
    critical: &[G3HookCriticalCommand],
    visiting: &mut Vec<String>,
) -> Option<(usize, String)> {
    let (function, function_absolute_base) = resolve_visible_function(
        local,
        root,
        command_name,
        call_line_no,
        absolute_base,
        root_line_no,
    )?;
    if visiting.iter().any(|name| name == &function.name) {
        return None;
    }

    visiting.push(function.name.clone());
    let found = first_fail_open_critical_command(
        &function.parsed_body,
        root,
        function_absolute_base,
        root_line_no,
        critical,
        visiting,
    );
    let _ = visiting.pop();
    found
}

/// `command_matches_critical` function.
fn command_matches_critical(command: &ResolvedCommand, critical: &[G3HookCriticalCommand]) -> bool {
    critical.iter().any(|expected| match expected {
        G3HookCriticalCommand::Binary(binary) => command.command_name() == binary,
        G3HookCriticalCommand::CargoSubcommand(subcommand) => {
            command.command_name() == "cargo" && first_cargo_subcommand(command) == Some(subcommand)
        }
    })
}

/// `first_cargo_subcommand` function.
fn first_cargo_subcommand(command: &ResolvedCommand) -> Option<&String> {
    let args = command.args();
    let mut index = 0usize;

    if args.get(index).is_some_and(|token| token.starts_with('+')) {
        index += 1;
    }

    while let Some(token) = args.get(index).map(String::as_str) {
        if !token.starts_with('-') {
            break;
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

    args.get(index)
}

/// `cargo_global_flag_takes_value` function.
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

/// `function_body_absolute_base` function.
const fn function_body_absolute_base(absolute_base: usize, function: &ShellFunction) -> usize {
    absolute_base
        + if function.body_starts_on_definition_line {
            function.line_no.saturating_sub(1)
        } else {
            function.line_no
        }
}

/// `resolve_visible_function` function.
fn resolve_visible_function<'a>(
    local: &'a ParsedShellScript,
    root: &'a ParsedShellScript,
    command_name: &str,
    local_line_no: usize,
    absolute_base: usize,
    root_line_no: usize,
) -> Option<(&'a ShellFunction, usize)> {
    if let Some(function) = local
        .functions
        .iter()
        .rev()
        .find(|function| function.name == command_name && function.line_no <= local_line_no)
    {
        return Some((
            function,
            function_body_absolute_base(absolute_base, function),
        ));
    }

    if std::ptr::eq(local, root) {
        return None;
    }

    root.functions
        .iter()
        .rev()
        .find(|function| function.name == command_name && function.line_no <= root_line_no)
        .map(|function| (function, function_body_absolute_base(1, function)))
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "API takes owned Vec to keep signature stable across the contract surface; downstream callers pass ownership"
)]
#[cfg(test)]
pub(crate) fn run_case(
    content: &str,
    requirements: Vec<g3rs_hooks_contract_types::G3HookRequirement>,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = FailOpenWrapperInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        requirements: &requirements,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
