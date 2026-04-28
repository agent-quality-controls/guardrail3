use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command_on_line_in_context};
use hook_shell_parser::types::{ParsedShellScript, ShellFunction};

use crate::inputs::FailOpenWrapperInput;

const ID: &str = "g3rs-hooks/no-fail-open-wrappers";

pub(crate) fn check(input: &FailOpenWrapperInput<'_>, results: &mut Vec<G3CheckResult>) {
    if let Some((line_no, command_text)) =
        first_fail_open_critical_command(input.parsed, input.parsed, 1, 0, &mut Vec::new())
    {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "critical hook command is fail-open".to_owned(),
            format!(
                "Critical hook command `{}` is softened by a fail-open wrapper.",
                command_text
            ),
            Some(input.rel_path.to_owned()),
            Some(line_no),
            false,
        ));
    }
}

fn first_fail_open_critical_command(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    absolute_base: usize,
    root_line_no: usize,
    visiting: &mut Vec<String>,
) -> Option<(usize, String)> {
    for line in &local.executable_lines {
        let absolute_line_no = absolute_line_no(absolute_base, line.line_no);
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
                is_guardrail_critical_command,
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
        visiting,
    );
    let _ = visiting.pop();
    found
}

fn is_guardrail_critical_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "g3rs"
        || command.command_name() == "gitleaks"
        || command.command_name() == "cargo-deny"
        || command.command_name() == "cargo-machete"
        || command.command_name() == "cargo-dupes"
        || (command.command_name() == "cargo" && cargo_subcommand_is_guardrail_critical(command))
}

fn cargo_subcommand_is_guardrail_critical(command: &ResolvedCommand) -> bool {
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

    matches!(
        args.get(index).map(String::as_str),
        Some("clippy" | "deny" | "test" | "machete" | "dupes")
    )
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

fn absolute_line_no(absolute_base: usize, local_line_no: usize) -> usize {
    absolute_base + local_line_no.saturating_sub(1)
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
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = FailOpenWrapperInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        requirements: &[],
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
