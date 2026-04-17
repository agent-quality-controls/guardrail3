use crate::compat::{G3CheckResult, G3Severity};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "RS-HOOKS-SOURCE-18";

pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    if let Some(raw_line) = first_exit_zero_line(input.parsed, &mut Vec::new()) {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "unconditional exit 0 bypass present".to_owned(),
            "Hook contains an executable `exit 0`, which can mask failures.".to_owned(),
            Some(input.rel_path.to_owned()),
            locate_line_no(input.parsed, &raw_line),
            false,
        ));
    } else {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "no unconditional exit 0 bypass".to_owned(),
                "Hook does not contain an executable `exit 0` bypass.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
    }
}

fn first_exit_zero_line(
    parsed: &hook_shell_parser::types::ParsedShellScript,
    visiting: &mut Vec<String>,
) -> Option<String> {
    for line in &parsed.executable_lines {
        if line.is_exit_zero {
            return Some(line.raw.to_owned());
        }
        if let Some(line_no) =
            called_function_exit_zero_line(parsed, &line.command_name, line.line_no, visiting)
        {
            return Some(line_no);
        }
    }

    None
}

fn called_function_exit_zero_line(
    parsed: &hook_shell_parser::types::ParsedShellScript,
    command_name: &str,
    call_line_no: usize,
    visiting: &mut Vec<String>,
) -> Option<String> {
    let function = parsed
        .functions
        .iter()
        .find(|function| function.name == command_name && function.line_no <= call_line_no)?;
    if visiting.iter().any(|name| name == &function.name) {
        return None;
    }

    visiting.push(function.name.to_owned());
    let nested = hook_shell_parser::parse_script(&function.body);
    let line_no = first_exit_zero_line(&nested, visiting);
    let _ = visiting.pop();
    line_no
}

fn locate_line_no(parsed: &hook_shell_parser::types::ParsedShellScript, raw_line: &str) -> Option<usize> {
    parsed
        .source_lines
        .iter()
        .find(|line| line.raw.trim() == raw_line.trim())
        .map(|line| line.line_no)
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: crate::facts::HookScriptKind::PreCommit,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
mod tests;
