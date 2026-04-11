use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command_on_line};

use crate::inputs::FailOpenWrapperInput;

const ID: &str = "HOOK-SHARED-21";

pub(crate) fn check(input: &FailOpenWrapperInput<'_>, results: &mut Vec<G3CheckResult>) {
    for line in input.executable_lines {
        if line.softened_by().is_none()
            || !any_resolved_command_on_line(
                input.parsed,
                line.raw(),
                line.line_no(),
                is_guardrail_critical_command,
            )
        {
            continue;
        }

        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "critical hook command is fail-open".to_owned(),
            format!(
                "Critical hook command `{}` is softened by a fail-open wrapper.",
                line.command_text()
            ),
            Some(input.rel_path.to_owned()),
            Some(line.line_no()),
            false,
        ));
    }
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

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = FailOpenWrapperInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        executable_lines: parsed.executable_lines(),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]

mod tests;
