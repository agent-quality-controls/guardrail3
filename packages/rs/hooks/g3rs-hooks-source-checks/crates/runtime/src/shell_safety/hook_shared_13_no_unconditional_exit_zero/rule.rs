use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::types::{ParsedShellScript, ShellFunction};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "RS-HOOKS-SOURCE-18";

pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    if let Some(line_no) = first_unconditional_exit_zero_line(input.parsed, &mut Vec::new()) {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "remove unconditional `exit 0` from `.githooks/pre-commit`".to_owned(),
            "`.githooks/pre-commit` contains an unconditional `exit 0` path that can force the hook to succeed before later checks run. Keep `exit 0` only for the final success path, not as an early bypass.".to_owned(),
            Some(input.rel_path.to_owned()),
            Some(line_no),
            false,
        ));
    } else {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "no unconditional `exit 0` bypass in `.githooks/pre-commit`".to_owned(),
                "`.githooks/pre-commit` does not contain an unconditional early `exit 0` path."
                    .to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
    }
}

fn first_unconditional_exit_zero_line(
    parsed: &ParsedShellScript,
    visiting: &mut Vec<String>,
) -> Option<usize> {
    if let Some(line_no) = first_top_level_exit_zero_line(parsed, visiting) {
        return Some(line_no);
    }

    None
}

fn first_top_level_exit_zero_line(
    parsed: &ParsedShellScript,
    visiting: &mut Vec<String>,
) -> Option<usize> {
    let function_ranges = function_line_ranges(parsed);
    let mut if_depth = 0usize;
    let mut case_depth = 0usize;
    let mut loop_depth = 0usize;

    for source_line in &parsed.source_lines {
        if line_in_function_body(source_line.line_no, function_ranges.as_slice()) {
            continue;
        }

        let trimmed = strip_inline_comment(&source_line.raw).trim();
        if trimmed.is_empty() || trimmed.starts_with("#!") {
            continue;
        }

        if closes_case_scope(trimmed) {
            case_depth = case_depth.saturating_sub(1);
            continue;
        }
        if closes_loop_scope(parsed, source_line.line_no, trimmed) {
            loop_depth = loop_depth.saturating_sub(1);
            continue;
        }
        if closes_if_scope(trimmed) {
            if_depth = if_depth.saturating_sub(1);
            continue;
        }

        if if_depth == 0 && case_depth == 0 && loop_depth == 0 {
            for line in parsed
                .executable_lines
                .iter()
                .filter(|line| line.line_no == source_line.line_no)
            {
                if line.is_exit_zero {
                    return Some(line.line_no);
                }
                if let Some(line_no) = called_function_exit_zero_line(
                    parsed,
                    &line.command_name,
                    line.line_no,
                    visiting,
                ) {
                    return Some(line_no);
                }
            }
        }

        if opens_case_scope(trimmed) {
            case_depth += 1;
            continue;
        }
        if opens_loop_scope(trimmed) {
            loop_depth += 1;
            continue;
        }
        if opens_if_scope(trimmed) {
            if_depth += 1;
        }
    }

    None
}

fn called_function_exit_zero_line(
    parsed: &ParsedShellScript,
    command_name: &str,
    call_line_no: usize,
    visiting: &mut Vec<String>,
) -> Option<usize> {
    let function = parsed
        .functions
        .iter()
        .find(|function| function.name == command_name && function.line_no <= call_line_no)?;
    if visiting.iter().any(|name| name == &function.name) {
        return None;
    }

    visiting.push(function.name.to_owned());
    let line_no = first_unconditional_exit_zero_line(&function.parsed_body, visiting)
        .map(|nested_line_no| absolute_function_body_line(function, nested_line_no));
    let _ = visiting.pop();
    line_no
}

fn absolute_function_body_line(function: &ShellFunction, nested_line_no: usize) -> usize {
    let body_start_line = if function.body_starts_on_definition_line {
        function.line_no
    } else {
        function.line_no + 1
    };
    body_start_line + nested_line_no.saturating_sub(1)
}

fn function_line_ranges(parsed: &ParsedShellScript) -> Vec<(usize, usize)> {
    parsed
        .functions
        .iter()
        .map(|function| {
            let body_line_count = function.parsed_body.source_lines.len();
            let end_line = if body_line_count == 0 {
                function.line_no
            } else {
                function.line_no + body_line_count
                    - usize::from(function.body_starts_on_definition_line)
            };
            (function.line_no, end_line)
        })
        .collect()
}

fn line_in_function_body(line_no: usize, ranges: &[(usize, usize)]) -> bool {
    ranges
        .iter()
        .any(|(start, end)| (*start..=*end).contains(&line_no))
}

fn strip_inline_comment(line: &str) -> &str {
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut prev_was_whitespace = true;

    for (idx, ch) in line.char_indices() {
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '#' if !single_quoted && !double_quoted && prev_was_whitespace => return &line[..idx],
            _ => {}
        }
        prev_was_whitespace = ch.is_whitespace();
    }

    line
}

fn opens_if_scope(line: &str) -> bool {
    line.starts_with("if ") && !closes_if_scope(line)
}

fn closes_if_scope(line: &str) -> bool {
    matches!(line, "fi" | "else")
        || line.starts_with("elif ")
        || line.ends_with("; fi")
        || line.ends_with(";fi")
}

fn opens_case_scope(line: &str) -> bool {
    line.starts_with("case ") && !closes_case_scope(line)
}

fn closes_case_scope(line: &str) -> bool {
    line == "esac" || line.ends_with("; esac") || line.ends_with(";esac")
}

fn opens_loop_scope(line: &str) -> bool {
    matches!(line.split_whitespace().next(), Some("for" | "while" | "until"))
}

fn closes_loop_scope(parsed: &ParsedShellScript, line_no: usize, line: &str) -> bool {
    if parsed
        .executable_lines
        .iter()
        .any(|executable| executable.line_no == line_no && executable.command_name == "done")
    {
        return true;
    }

    line == "done" || line.ends_with("; done") || line.ends_with(";done")
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
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
