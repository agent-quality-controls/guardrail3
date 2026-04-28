use crate::shell_ast::{self, HeredocTerminator};
use crate::types::ExecutableLine;

pub(super) fn collect_logical_lines(content: &str) -> Vec<(usize, String)> {
    let mut logical_lines = Vec::new();
    let mut continuation_start: Option<usize> = None;
    let mut continuation_line_no: Option<usize> = None;
    let mut offset = 0usize;
    let mut heredoc_terminator: Option<HeredocTerminator> = None;

    for (index, raw) in content.lines().enumerate() {
        let line_no = index + 1;
        let line_start = offset;
        let line_end = line_start + raw.len();
        offset = line_end + 1;

        if let Some(terminator) = heredoc_terminator.as_ref() {
            let candidate = if terminator.strip_tabs {
                raw.trim_start_matches('\t').trim()
            } else {
                raw.trim()
            };
            if candidate == terminator.value {
                heredoc_terminator = None;
            }
            continue;
        }

        if continuation_start.is_none() {
            continuation_start = Some(line_start);
            continuation_line_no = Some(line_no);
        }

        if raw.trim_end().ends_with('\\') {
            continue;
        }

        let start = continuation_start.unwrap_or(line_start);
        let end = line_end;
        let logical = &content[start..end];
        if let Some(terminator) = heredoc_delimiter(logical) {
            heredoc_terminator = Some(terminator);
        }
        logical_lines.push((continuation_line_no.unwrap_or(line_no), logical.to_owned()));
        continuation_start = None;
        continuation_line_no = None;
    }

    if let Some(start) = continuation_start
        && start < content.len()
    {
        logical_lines.push((
            continuation_line_no.unwrap_or(1),
            content[start..].to_owned(),
        ));
    }

    logical_lines
}

pub(super) fn parse_executable_line(raw: &str, line_no: usize) -> Option<ExecutableLine> {
    let trimmed = strip_inline_comment(raw).trim();
    if trimmed.is_empty() || trimmed.starts_with('#') || is_shell_control_line(trimmed) {
        return None;
    }
    if is_constant_condition_control_line(trimmed) {
        return None;
    }
    if let Some(inner_command) = assignment_command_substitution(trimmed) {
        let softened_by = crate::fail_open::command_substitution_assignment_wrapper(trimmed)
            .or_else(|| crate::fail_open::detect_fail_open_wrapper(&inner_command));
        let command_text = extract_command_segment(&inner_command);
        let command_name = leading_command_name(&command_text)?;

        return Some(ExecutableLine {
            line_no,
            raw: raw.to_owned(),
            command_text,
            command_name: command_name.clone(),
            softened_by,
            is_dispatcher_syntax: is_dispatcher_command(&command_name),
            is_exit_zero: command_name == "exit" && argument_starts_with_zero(&inner_command),
        });
    }
    if is_standalone_assignment(trimmed) {
        return None;
    }

    let softened_by = crate::fail_open::detect_fail_open_wrapper(trimmed);
    let command_text = extract_command_segment(trimmed);
    let command_name = leading_command_name(&command_text)?;

    Some(ExecutableLine {
        line_no,
        raw: raw.to_owned(),
        command_text,
        command_name: command_name.clone(),
        softened_by,
        is_dispatcher_syntax: is_dispatcher_command(&command_name),
        is_exit_zero: command_name == "exit" && argument_starts_with_zero(trimmed),
    })
}

pub(super) fn parse_executable_segments(raw: &str, line_no: usize) -> Vec<ExecutableLine> {
    shell_ast::command_segments(raw)
        .into_iter()
        .filter(|segment| {
            !matches!(segment.operator_before, Some("||"))
                || !crate::fail_open::is_fail_open_wrapper_command(&segment.text)
        })
        .filter_map(|segment| parse_executable_line(&segment.text, line_no))
        .collect()
}

pub(super) fn single_line_constant_if_taken_branch(raw: &str) -> Option<Option<&str>> {
    let trimmed = strip_inline_comment(raw).trim();
    if !trimmed.starts_with("if ") || !trimmed.contains("then") || !trimmed.contains("fi") {
        return None;
    }

    let after_if = trimmed.strip_prefix("if ")?.trim();
    let (condition, after_then) = after_if.split_once("then")?;
    let status = constant_shell_status(condition.trim_end_matches(';').trim())?;
    let after_then = after_then.trim();

    if let Some((then_branch, elif_branch)) = after_then.split_once("elif") {
        if status {
            return Some(Some(branch_before_fi(then_branch).trim()));
        }
        let (elif_condition, elif_after_then) = elif_branch.split_once("then")?;
        let elif_status = constant_shell_status(elif_condition.trim_end_matches(';').trim())?;
        if let Some((elif_then_branch, else_branch)) = elif_after_then.split_once("else") {
            return Some(if elif_status {
                Some(branch_before_fi(elif_then_branch).trim())
            } else {
                Some(branch_before_fi(else_branch).trim())
            });
        }
        return Some(if elif_status {
            Some(branch_before_fi(elif_after_then).trim())
        } else {
            None
        });
    }

    if let Some((then_branch, else_branch)) = after_then.split_once("else") {
        let branch = if status { then_branch } else { else_branch };
        return Some(Some(branch_before_fi(branch).trim()));
    }

    Some(if status {
        Some(branch_before_fi(after_then).trim())
    } else {
        None
    })
}

fn branch_before_fi(branch: &str) -> &str {
    branch
        .split_once("fi")
        .map(|(before, _)| before)
        .unwrap_or(branch)
        .trim_end_matches(';')
        .trim()
}

pub(super) fn extract_command_segment(line: &str) -> String {
    let line = strip_inline_comment(line).trim();
    let segments = shell_ast::command_segments(line);
    if segments.is_empty() {
        return line.to_owned();
    }

    let relevant = segments
        .iter()
        .take_while(|segment| {
            !matches!(segment.operator_before, Some("||"))
                || !crate::fail_open::is_fail_open_wrapper_command(&segment.text)
        })
        .collect::<Vec<_>>();
    if relevant.is_empty() {
        return String::new();
    }

    if relevant
        .iter()
        .any(|segment| matches!(segment.operator_before, Some("|")))
    {
        return relevant
            .iter()
            .map(|segment| strip_leading_negation(&segment.text))
            .collect::<Vec<_>>()
            .join(" | ");
    }

    if let Some(last_and) = relevant
        .iter()
        .rev()
        .find(|segment| matches!(segment.operator_before, Some("&&")))
    {
        return strip_leading_negation(&last_and.text).to_owned();
    }

    relevant
        .first()
        .map(|segment| strip_leading_negation(&segment.text).to_owned())
        .unwrap_or_default()
}

fn strip_leading_negation(command_text: &str) -> &str {
    command_text
        .trim_start()
        .strip_prefix('!')
        .map(str::trim_start)
        .unwrap_or(command_text)
}

fn heredoc_delimiter(line: &str) -> Option<HeredocTerminator> {
    shell_ast::heredoc_terminator(line)
}

pub(super) fn strip_inline_comment(line: &str) -> &str {
    shell_ast::strip_inline_comment(line)
}

fn leading_command_name(command_text: &str) -> Option<String> {
    shell_ast::leading_command_name(command_text)
}

fn is_dispatcher_command(command_name: &str) -> bool {
    matches!(command_name, "source" | "." | "run-parts")
}

fn argument_starts_with_zero(command_text: &str) -> bool {
    shell_ast::shell_words(command_text)
        .get(1)
        .is_some_and(|argument| argument == "0")
}

fn is_shell_control_line(line: &str) -> bool {
    matches!(
        line,
        "then" | "do" | "done" | "fi" | "esac" | "{" | "}" | "else"
    ) || line.starts_with("for ")
        || line.starts_with("while ")
        || line.starts_with("case ")
}

fn is_constant_condition_control_line(line: &str) -> bool {
    if let Some(condition) = line.strip_prefix("if ").and_then(|rest| {
        rest.split_once("then")
            .map(|(condition, _)| condition.trim())
    }) {
        return constant_shell_status(condition).is_some();
    }

    if let Some(condition) = line.strip_prefix("elif ").and_then(|rest| {
        rest.split_once("then")
            .map(|(condition, _)| condition.trim())
    }) {
        return constant_shell_status(condition).is_some();
    }

    false
}

fn is_standalone_assignment(line: &str) -> bool {
    shell_ast::is_standalone_assignment(line)
}

fn assignment_command_substitution(line: &str) -> Option<String> {
    shell_ast::command_substitution_assignment(line)
}

pub(super) fn is_function_definition_line(line: &str) -> bool {
    let line = strip_inline_comment(line).trim();
    if !line.contains('{') {
        return false;
    }

    line.strip_prefix("function ")
        .is_some_and(|rest| rest.split_whitespace().next().is_some())
        || line
            .split_once("()")
            .is_some_and(|(name, rest)| !name.trim().is_empty() && rest.contains('{'))
}

pub(super) fn function_definition_name(line: &str) -> Option<String> {
    let line = strip_inline_comment(line).trim();
    if let Some(rest) = line.strip_prefix("function ") {
        return rest
            .split_whitespace()
            .next()
            .map(|name| name.trim_end_matches("()").to_owned());
    }
    line.split_once("()")
        .map(|(name, _)| name.trim().to_owned())
        .filter(|name| !name.is_empty())
}

pub(super) fn initial_function_body_fragment(line: &str) -> String {
    let Some((_, rest)) = line.split_once('{') else {
        return String::new();
    };
    trim_trailing_function_closer(rest.trim_start())
}

pub(super) fn append_function_body_line(body: &mut String, raw: &str) {
    body.push_str(raw);
    body.push('\n');
}

fn trim_trailing_function_closer(body: &str) -> String {
    let trimmed = strip_inline_comment(body).trim_end();
    let Some(close_index) = function_definition_closer_index(trimmed) else {
        return String::new();
    };
    let stripped = trimmed[..close_index].trim_end();
    if stripped.is_empty() {
        String::new()
    } else {
        format!("{stripped}\n")
    }
}

pub(super) fn inline_command_after_function_definition(line: &str) -> Option<&str> {
    let line = strip_inline_comment(line);
    let (_, rest) = line.split_once('{')?;
    let close_index = function_definition_closer_index(rest.trim_start())?;
    let tail = rest.trim_start()[close_index + '}'.len_utf8()..]
        .trim_start_matches(|c: char| c == ';' || c.is_whitespace());
    (!tail.is_empty()).then_some(tail)
}

fn function_definition_closer_index(fragment: &str) -> Option<usize> {
    let chars: Vec<(usize, char)> = fragment.char_indices().collect();
    let mut brace_depth = 1usize;
    let mut contexts = vec![ShellTailContext::Function];
    let mut index = 0usize;
    let mut escaped = false;

    while let Some((idx, ch)) = chars.get(index).copied() {
        if escaped {
            escaped = false;
            index += 1;
            continue;
        }

        if ch == '\\' && !matches!(contexts.last(), Some(ShellTailContext::SingleQuote)) {
            escaped = true;
            index += 1;
            continue;
        }

        match contexts.last().copied() {
            Some(ShellTailContext::SingleQuote) => {
                if ch == '\'' {
                    let _ = contexts.pop();
                }
            }
            Some(ShellTailContext::DoubleQuote) => {
                if ch == '"' {
                    let _ = contexts.pop();
                } else if ch == '$'
                    && let Some(next) = start_shell_tail_context(chars.as_slice(), index)
                {
                    contexts.push(next);
                    index += 2;
                    continue;
                }
            }
            Some(ShellTailContext::CommandSubstitution) => {
                if ch == '\'' {
                    contexts.push(ShellTailContext::SingleQuote);
                } else if ch == '"' {
                    contexts.push(ShellTailContext::DoubleQuote);
                } else if ch == '$'
                    && let Some(next) = start_shell_tail_context(chars.as_slice(), index)
                {
                    contexts.push(next);
                    index += 2;
                    continue;
                } else if ch == ')' {
                    let _ = contexts.pop();
                }
            }
            Some(ShellTailContext::ParameterExpansion) => {
                if ch == '\'' {
                    contexts.push(ShellTailContext::SingleQuote);
                } else if ch == '"' {
                    contexts.push(ShellTailContext::DoubleQuote);
                } else if ch == '$'
                    && let Some(next) = start_shell_tail_context(chars.as_slice(), index)
                {
                    contexts.push(next);
                    index += 2;
                    continue;
                } else if ch == '}' {
                    let _ = contexts.pop();
                }
            }
            Some(ShellTailContext::Function) | None => match ch {
                '\'' => contexts.push(ShellTailContext::SingleQuote),
                '"' => contexts.push(ShellTailContext::DoubleQuote),
                '$' => {
                    if let Some(next) = start_shell_tail_context(chars.as_slice(), index) {
                        contexts.push(next);
                        index += 2;
                        continue;
                    }
                }
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth = brace_depth.saturating_sub(1);
                    if brace_depth == 0 {
                        return Some(idx);
                    }
                }
                _ => {}
            },
        }

        index += 1;
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShellTailContext {
    Function,
    SingleQuote,
    DoubleQuote,
    CommandSubstitution,
    ParameterExpansion,
}

fn start_shell_tail_context(chars: &[(usize, char)], index: usize) -> Option<ShellTailContext> {
    let next = chars.get(index + 1)?.1;
    match next {
        '(' => Some(ShellTailContext::CommandSubstitution),
        '{' => Some(ShellTailContext::ParameterExpansion),
        _ => None,
    }
}

pub(super) fn function_scope_depth_after_definition(line: &str) -> usize {
    brace_delta(line).max(0) as usize
}

pub(super) fn update_function_scope_depth(depth: usize, line: &str) -> usize {
    depth.saturating_add_signed(brace_delta(line))
}

pub(super) fn starts_dead_if_scope(line: &str) -> bool {
    let Some(condition) = line.strip_prefix("if ").and_then(|rest| {
        rest.split_once("then")
            .map(|(condition, _)| condition.trim())
    }) else {
        return false;
    };

    matches!(constant_shell_status(condition), Some(false))
}

pub(super) fn dead_if_scope_depth_after_start(line: &str) -> usize {
    if line.contains("fi") { 0 } else { 1 }
}

pub(super) fn update_dead_if_depth(depth: usize, line: &str) -> usize {
    let mut next = depth;
    let trimmed = strip_inline_comment(line).trim();
    if depth == 1 && trimmed == "else" {
        return 0;
    }
    if depth == 1
        && trimmed.starts_with("elif ")
        && trimmed.contains("then")
        && matches!(constant_shell_status(elif_condition(trimmed)), Some(true))
    {
        return 0;
    }
    if trimmed.starts_with("if ") && trimmed.contains("then") {
        next += 1;
    }
    if is_fi_line(trimmed) {
        next = next.saturating_sub(1);
    }
    next
}

pub(super) fn starts_live_true_if_scope(line: &str) -> bool {
    let Some(condition) = line.strip_prefix("if ").and_then(|rest| {
        rest.split_once("then")
            .map(|(condition, _)| condition.trim())
    }) else {
        return false;
    };

    matches!(constant_shell_status(condition), Some(true))
}

pub(super) fn starts_dead_alternate_if_branch(line: &str) -> bool {
    let trimmed = strip_inline_comment(line).trim();
    trimmed == "else" || (trimmed.starts_with("elif ") && trimmed.contains("then"))
}

pub(super) fn update_dead_else_depth(depth: usize, line: &str) -> usize {
    let mut next = depth;
    let trimmed = strip_inline_comment(line).trim();
    if trimmed.starts_with("if ") && trimmed.contains("then") {
        next += 1;
    }
    if is_fi_line(trimmed) {
        next = next.saturating_sub(1);
    }
    next
}

pub(super) fn starts_dead_loop_scope(line: &str) -> bool {
    if let Some(condition) = line
        .strip_prefix("while ")
        .and_then(|rest| rest.split_once("do").map(|(condition, _)| condition.trim()))
    {
        return matches!(constant_shell_status(condition), Some(false));
    }
    if let Some(condition) = line
        .strip_prefix("until ")
        .and_then(|rest| rest.split_once("do").map(|(condition, _)| condition.trim()))
    {
        return matches!(constant_shell_status(condition), Some(true));
    }
    false
}

pub(super) fn dead_loop_scope_depth_after_start(line: &str) -> usize {
    if line.contains("done") { 0 } else { 1 }
}

pub(super) fn update_dead_loop_depth(depth: usize, line: &str) -> usize {
    let mut next = depth;
    let trimmed = strip_inline_comment(line).trim();
    if (trimmed.starts_with("while ")
        || trimmed.starts_with("until ")
        || trimmed.starts_with("for "))
        && trimmed.contains("do")
    {
        next += 1;
    }
    if trimmed == "done" || trimmed.ends_with("; done") {
        next = next.saturating_sub(1);
    }
    next
}

fn elif_condition(line: &str) -> &str {
    line.strip_prefix("elif ")
        .and_then(|rest| {
            rest.split_once("then")
                .map(|(condition, _)| condition.trim())
        })
        .unwrap_or_default()
}

pub(super) fn is_fi_line(line: &str) -> bool {
    line == "fi" || line.ends_with("; fi")
}

fn brace_delta(line: &str) -> isize {
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut delta = 0isize;

    for ch in strip_inline_comment(line).chars() {
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '{' if !single_quoted && !double_quoted => delta += 1,
            '}' if !single_quoted && !double_quoted => delta -= 1,
            _ => {}
        }
    }

    delta
}

pub(super) fn constant_shell_status(segment: &str) -> Option<bool> {
    let mut segment = segment.trim().trim_end_matches(';').trim();
    let mut negated = false;

    while let Some(stripped) = segment.strip_prefix('!') {
        negated = !negated;
        segment = stripped.trim();
    }

    segment = segment.trim_matches(|c: char| c == '(' || c == ')' || c == '{' || c == '}');
    let status = match segment {
        "true" | ":" => Some(true),
        "false" => Some(false),
        value if value.starts_with("exit ") => Some(value.split_whitespace().nth(1) == Some("0")),
        _ => None,
    }?;

    Some(if negated { !status } else { status })
}
