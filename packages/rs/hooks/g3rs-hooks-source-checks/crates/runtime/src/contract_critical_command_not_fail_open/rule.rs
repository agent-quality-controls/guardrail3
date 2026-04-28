use std::collections::BTreeSet;

use crate::compat::{G3CheckResult, G3Severity};
use crate::inputs::FailOpenWrapperInput;
use g3rs_hooks_contract_types::G3HookCriticalCommand;
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command_on_line_in_context};
use hook_shell_parser::types::{ParsedShellScript, ShellFunction};

const ID: &str = "g3rs-hooks/contract-critical-command-not-fail-open";

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
            G3Severity::Warn,
            "contract-critical hook command is fail-open".to_owned(),
            format!(
                "Critical hook command `{}` is softened by a fail-open wrapper. Critical commands come from family hook contracts; do not hide failures with `|| true`, `|| echo ...`, or equivalent wrappers.",
                command_text
            ),
            Some(input.rel_path.to_owned()),
            Some(line_no),
            false,
        ));
    }
}

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
            return Some((absolute_line_no, line.command_text.to_owned()));
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

    visiting.push(function.name.to_owned());
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

fn command_matches_critical(command: &ResolvedCommand, critical: &[G3HookCriticalCommand]) -> bool {
    critical.iter().any(|expected| match expected {
        G3HookCriticalCommand::Binary(binary) => command.command_name() == binary,
        G3HookCriticalCommand::CargoSubcommand(subcommand) => {
            command.command_name() == "cargo" && first_cargo_subcommand(command) == Some(subcommand)
        }
    })
}

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

fn function_body_absolute_base(absolute_base: usize, function: &ShellFunction) -> usize {
    absolute_base
        + if function.body_starts_on_definition_line {
            function.line_no.saturating_sub(1)
        } else {
            function.line_no
        }
}

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
