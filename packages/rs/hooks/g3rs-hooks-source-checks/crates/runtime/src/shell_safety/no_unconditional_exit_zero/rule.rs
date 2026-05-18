#![expect(
    clippy::arithmetic_side_effects,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::option_if_let_else,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::string_slice,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::too_many_lines,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::type_complexity,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command_on_line_in_context};
use hook_shell_parser::types::{ParsedShellScript, ShellFunction};

use crate::inputs::ExecutableCommandContextInput;

/// `ID` constant.
const ID: &str = "g3rs-hooks/no-unconditional-exit-zero";

/// `check` function.
pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    if let Some(line_no) =
        first_unconditional_exit_zero_line(input.parsed, input.parsed, 1, 0, &mut Vec::new())
    {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            // Reason: an unconditional `exit 0` bypass is a gate-defeating shape; it must
            // make the gate fail.
            G3Severity::Error,
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

/// `first_unconditional_exit_zero_line` function.
fn first_unconditional_exit_zero_line(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    absolute_base: usize,
    root_line_no: usize,
    visiting: &mut Vec<String>,
) -> Option<usize> {
    if let Some(line_no) =
        first_top_level_exit_zero_line(local, root, absolute_base, root_line_no, visiting)
    {
        return Some(line_no);
    }

    None
}

#[expect(
    clippy::excessive_nesting,
    reason = "shell script parser is inherently iterative with byte-walking loops"
)]
/// `first_top_level_exit_zero_line` function.
fn first_top_level_exit_zero_line(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    absolute_base: usize,
    root_line_no: usize,
    visiting: &mut Vec<String>,
) -> Option<usize> {
    let function_ranges = function_line_ranges(local);
    let mut if_depth = 0usize;
    let mut case_depth = 0usize;
    let mut loop_depth = 0usize;

    for source_line in &local.source_lines {
        let trimmed = strip_inline_comment(&source_line.raw).trim();
        if trimmed.is_empty() || trimmed.starts_with("#!") {
            continue;
        }

        let absolute_line_no = absolute_line_no(absolute_base, source_line.line_no);
        let visible_root_line_no = if std::ptr::eq(local, root) {
            absolute_line_no
        } else {
            root_line_no
        };

        if is_function_definition_line(local, source_line.line_no) {
            if if_depth == 0 && case_depth == 0 && loop_depth == 0 {
                if let Some(tail) = function_definition_tail(&source_line.raw) {
                    if let Some(line_no) = scan_tail_for_unconditional_exit_zero(
                        local,
                        root,
                        tail,
                        source_line.line_no,
                        absolute_base,
                        visible_root_line_no,
                        visiting,
                    ) {
                        return Some(line_no);
                    }
                }
            }

            continue;
        }

        if line_in_function_body(source_line.line_no, function_ranges.as_slice()) {
            continue;
        }

        if let Some(tail) = same_line_scoped_control_flow_tail(trimmed) {
            if let Some(line_no) = scan_tail_for_unconditional_exit_zero(
                local,
                root,
                tail,
                source_line.line_no,
                absolute_base,
                visible_root_line_no,
                visiting,
            ) {
                return Some(line_no);
            }
            continue;
        }

        if closes_case_scope(trimmed) {
            case_depth = case_depth.saturating_sub(1);
        }
        if closes_loop_scope(local, source_line.line_no, trimmed) {
            loop_depth = loop_depth.saturating_sub(1);
        }
        if closes_if_scope(trimmed) {
            if_depth = if_depth.saturating_sub(1);
        }

        if if_depth == 0
            && case_depth == 0
            && loop_depth == 0
            && !opens_case_scope(trimmed)
            && !opens_loop_scope(trimmed)
            && !opens_if_scope(trimmed)
        {
            if is_same_line_scoped_control_flow(trimmed) {
                continue;
            }

            for line in local
                .executable_lines
                .iter()
                .filter(|line| line.line_no == source_line.line_no)
            {
                if let Some(line_no) = called_function_exit_zero_line(
                    local,
                    root,
                    &line.command_name,
                    line.line_no,
                    absolute_base,
                    visible_root_line_no,
                    visiting,
                ) {
                    return Some(line_no);
                }
            }

            if line_contains_exit_zero_path(
                local,
                root,
                &source_line.raw,
                source_line.line_no,
                visible_root_line_no,
            ) {
                return Some(absolute_line_no);
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

/// `called_function_exit_zero_line` function.
fn called_function_exit_zero_line(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    command_name: &str,
    call_line_no: usize,
    absolute_base: usize,
    root_line_no: usize,
    visiting: &mut Vec<String>,
) -> Option<usize> {
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
    let line_no = first_unconditional_exit_zero_line(
        &function.parsed_body,
        root,
        function_absolute_base,
        root_line_no,
        visiting,
    );
    let _ = visiting.pop();
    line_no
}

/// `called_function_exit_zero_line_from_tail` function.
fn called_function_exit_zero_line_from_tail(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    tail: &str,
    call_line_no: usize,
    absolute_base: usize,
    root_line_no: usize,
    visiting: &mut Vec<String>,
) -> Option<usize> {
    let tail_script = hook_shell_parser::parse_script(tail);
    for executable in &tail_script.executable_lines {
        if let Some(line_no) = called_function_exit_zero_line(
            local,
            root,
            &executable.command_name,
            call_line_no,
            absolute_base,
            root_line_no,
            visiting,
        ) {
            return Some(line_no);
        }
    }

    None
}

/// `function_line_ranges` function.
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

/// `line_in_function_body` function.
fn line_in_function_body(line_no: usize, ranges: &[(usize, usize)]) -> bool {
    ranges
        .iter()
        .any(|(start, end)| (*start..=*end).contains(&line_no))
}

/// `is_function_definition_line` function.
fn is_function_definition_line(parsed: &ParsedShellScript, line_no: usize) -> bool {
    parsed
        .functions
        .iter()
        .any(|function| function.line_no == line_no)
}

/// `function_definition_tail` function.
fn function_definition_tail(line: &str) -> Option<&str> {
    let line = strip_inline_comment(line);
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut brace_depth = 0usize;
    let mut saw_open_brace = false;

    for (idx, ch) in line.char_indices() {
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '{' if !single_quoted && !double_quoted => {
                saw_open_brace = true;
                brace_depth += 1;
            }
            '}' if !single_quoted && !double_quoted && saw_open_brace => {
                brace_depth = brace_depth.saturating_sub(1);
                if brace_depth == 0 {
                    let tail = line[idx + ch.len_utf8()..]
                        .trim_start_matches(|c: char| c == ';' || c.is_whitespace());
                    return (!tail.is_empty()).then_some(tail);
                }
            }
            _ => {}
        }
    }

    None
}

/// `line_contains_exit_zero_path` function.
fn line_contains_exit_zero_path(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    raw: &str,
    line_no: usize,
    root_line_no: usize,
) -> bool {
    any_resolved_command_on_line_in_context(
        local,
        root,
        raw,
        line_no,
        root_line_no,
        |command: &ResolvedCommand| {
            command.command_name() == "exit"
                && command.command_text().split_whitespace().nth(1) == Some("0")
        },
    )
}

/// `scan_tail_for_unconditional_exit_zero` function.
fn scan_tail_for_unconditional_exit_zero(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    tail: &str,
    line_no: usize,
    absolute_base: usize,
    root_line_no: usize,
    visiting: &mut Vec<String>,
) -> Option<usize> {
    if let Some(line_no) = called_function_exit_zero_line_from_tail(
        local,
        root,
        tail,
        line_no,
        absolute_base,
        root_line_no,
        visiting,
    ) {
        return Some(line_no);
    }

    if line_contains_exit_zero_path(local, root, tail, line_no, root_line_no) {
        return Some(absolute_line_no(absolute_base, line_no));
    }

    None
}

/// `strip_inline_comment` function.
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

/// `opens_if_scope` function.
fn opens_if_scope(line: &str) -> bool {
    starts_shell_keyword(line, "if") && !closes_if_scope(line)
}

/// `closes_if_scope` function.
fn closes_if_scope(line: &str) -> bool {
    matches!(line, "fi" | "else")
        || starts_shell_keyword(line, "fi")
        || line.starts_with("elif ")
        || line.ends_with("; fi")
        || line.ends_with(";fi")
}

/// `opens_case_scope` function.
fn opens_case_scope(line: &str) -> bool {
    starts_shell_keyword(line, "case") && !closes_case_scope(line)
}

/// `closes_case_scope` function.
fn closes_case_scope(line: &str) -> bool {
    starts_shell_keyword(line, "esac")
        || line.ends_with("; esac")
        || line.ends_with(";esac")
        || contains_shell_keyword(line, "esac")
}

/// `is_same_line_scoped_control_flow` function.
fn is_same_line_scoped_control_flow(line: &str) -> bool {
    (starts_shell_keyword(line, "if")
        && line.contains("then")
        && closes_if_scope(line)
        && line.ends_with("fi"))
        || (starts_shell_keyword(line, "case")
            && line.contains(" in ")
            && closes_case_scope(line)
            && line.ends_with("esac"))
}

/// `opens_loop_scope` function.
fn opens_loop_scope(line: &str) -> bool {
    matches!(
        line.split_whitespace().next(),
        Some("for" | "while" | "until")
    ) || starts_time_prefixed_loop_scope(line)
}

/// `same_line_scoped_control_flow_tail` function.
fn same_line_scoped_control_flow_tail(line: &str) -> Option<&str> {
    if starts_shell_keyword(line, "if") && line.contains("then") {
        return tail_after_shell_keyword(line, "fi");
    }
    if starts_shell_keyword(line, "case") && line.contains(" in ") {
        return tail_after_shell_keyword(line, "esac");
    }
    if opens_loop_scope(line) {
        return tail_after_shell_keyword(line, "done");
    }

    None
}

/// `closes_loop_scope` function.
fn closes_loop_scope(parsed: &ParsedShellScript, line_no: usize, line: &str) -> bool {
    if parsed
        .executable_lines
        .iter()
        .any(|executable| executable.line_no == line_no && executable.command_name == "done")
    {
        return true;
    }

    starts_shell_keyword(line, "done") || line.ends_with("; done") || line.ends_with(";done")
}

/// `starts_shell_keyword` function.
fn starts_shell_keyword(line: &str, keyword: &str) -> bool {
    let Some(rest) = line.strip_prefix(keyword) else {
        return false;
    };
    rest.is_empty()
        || rest.starts_with(|c: char| c.is_whitespace() || matches!(c, ';' | '&' | '|' | '<' | '>'))
}

/// `starts_time_prefixed_loop_scope` function.
fn starts_time_prefixed_loop_scope(line: &str) -> bool {
    let Some(rest) = line.strip_prefix("time ") else {
        return false;
    };
    let rest = if let Some(after_flag) = rest.strip_prefix("-p ") {
        after_flag
    } else {
        rest
    };
    matches!(
        rest.split_whitespace().next(),
        Some("for" | "while" | "until")
    )
}

/// `contains_shell_keyword` function.
fn contains_shell_keyword(line: &str, keyword: &str) -> bool {
    let mut search_start = 0usize;

    while let Some(relative_index) = line[search_start..].find(keyword) {
        let start = search_start + relative_index;
        let end = start + keyword.len();

        let before_ok = line[..start]
            .chars()
            .last()
            .is_none_or(|ch| ch.is_whitespace() || matches!(ch, ';' | '&' | '|' | '<' | '>'));
        let after_ok = line[end..]
            .chars()
            .next()
            .is_none_or(|ch| ch.is_whitespace() || matches!(ch, ';' | '&' | '|' | '<' | '>'));

        if before_ok && after_ok {
            return true;
        }

        search_start = end;
    }

    false
}

/// `tail_after_shell_keyword` function.
fn tail_after_shell_keyword<'a>(line: &'a str, keyword: &str) -> Option<&'a str> {
    let mut search_end = line.len();
    while let Some(relative_index) = line[..search_end].rfind(keyword) {
        let start = relative_index;
        let end = start + keyword.len();
        let before_ok = line[..start]
            .chars()
            .last()
            .is_none_or(|ch| ch.is_whitespace() || matches!(ch, ';' | '&' | '|' | '<' | '>'));
        let after_ok = line[end..]
            .chars()
            .next()
            .is_none_or(|ch| ch.is_whitespace() || matches!(ch, ';' | '&' | '|' | '<' | '>'));

        if before_ok && after_ok {
            let tail = line[end..].trim_start_matches(|c: char| c == ';' || c.is_whitespace());
            return Some(tail);
        }

        search_end = start;
    }

    None
}

/// `absolute_line_no` function.
const fn absolute_line_no(absolute_base: usize, local_line_no: usize) -> usize {
    absolute_base + local_line_no.saturating_sub(1)
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
