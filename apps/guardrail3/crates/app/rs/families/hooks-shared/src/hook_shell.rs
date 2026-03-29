#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailOpenWrapper<'a> {
    True,
    NoOp,
    Echo(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutableLine<'a> {
    pub line_no: usize,
    pub raw: &'a str,
    pub command_text: &'a str,
    pub command_name: &'a str,
    pub softened_by: Option<FailOpenWrapper<'a>>,
    pub is_dispatcher_syntax: bool,
    pub is_exit_zero: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedShellScript<'a> {
    pub shebang: Option<&'a str>,
    pub executable_lines: Vec<ExecutableLine<'a>>,
    pub functions: Vec<ShellFunction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellFunction {
    pub name: String,
    pub line_no: usize,
    pub body: String,
}

#[must_use]
pub fn parse_script(content: &str) -> ParsedShellScript<'_> {
    let mut shebang = None;
    let mut executable_lines = Vec::new();
    let mut functions = Vec::new();
    let logical_lines = collect_logical_lines(content);
    let mut function_brace_depth = 0usize;
    let mut current_function: Option<ShellFunction> = None;
    let mut dead_if_depth = 0usize;
    let mut dead_else_depth = 0usize;
    let mut dead_loop_depth = 0usize;
    let mut live_true_if_depth = 0usize;

    for (line_no, raw) in logical_lines {
        let trimmed = raw.trim();
        if line_no == 1 && trimmed.starts_with("#!") {
            shebang = Some(trimmed);
            continue;
        }
        if let Some(current) = current_function.as_mut() {
            append_function_body_line(current, raw);
            function_brace_depth = update_function_scope_depth(function_brace_depth, trimmed);
            if function_brace_depth == 0
                && let Some(function) = current_function.take()
            {
                functions.push(function);
            }
            continue;
        }
        if let Some(taken_branch) = single_line_constant_if_taken_branch(raw) {
            if let Some(executable) =
                taken_branch.and_then(|branch| parse_executable_line(branch, line_no))
            {
                executable_lines.push(executable);
            }
            continue;
        }
        if dead_else_depth > 0 {
            dead_else_depth = update_dead_else_depth(dead_else_depth, trimmed);
            continue;
        }
        if function_brace_depth > 0 {
            function_brace_depth = update_function_scope_depth(function_brace_depth, trimmed);
            continue;
        }
        if dead_if_depth > 0 {
            dead_if_depth = update_dead_if_depth(dead_if_depth, trimmed);
            continue;
        }
        if dead_loop_depth > 0 {
            dead_loop_depth = update_dead_loop_depth(dead_loop_depth, trimmed);
            continue;
        }
        if is_function_definition_line(trimmed) {
            function_brace_depth = function_scope_depth_after_definition(trimmed);
            let function = ShellFunction {
                name: function_definition_name(trimmed).unwrap_or_default(),
                line_no,
                body: initial_function_body_fragment(raw),
            };
            if function_brace_depth == 0 {
                functions.push(function);
                if let Some(tail) = inline_command_after_function_definition(raw)
                    && let Some(executable) = parse_executable_line(tail, line_no)
                {
                    executable_lines.push(executable);
                }
            } else {
                current_function = Some(function);
            }
            continue;
        }
        if starts_dead_if_scope(trimmed) {
            dead_if_depth = dead_if_scope_depth_after_start(trimmed);
            continue;
        }
        if live_true_if_depth > 0 && starts_dead_alternate_if_branch(trimmed) {
            live_true_if_depth = live_true_if_depth.saturating_sub(1);
            dead_else_depth = 1;
            continue;
        }
        if starts_live_true_if_scope(trimmed) {
            live_true_if_depth += 1;
        } else if live_true_if_depth > 0 && is_fi_line(trimmed) {
            live_true_if_depth = live_true_if_depth.saturating_sub(1);
        }
        if starts_dead_loop_scope(trimmed) {
            dead_loop_depth = dead_loop_scope_depth_after_start(trimmed);
            continue;
        }
        let Some(executable) = parse_executable_line(raw, line_no) else {
            continue;
        };
        executable_lines.push(executable);
    }

    ParsedShellScript {
        shebang,
        executable_lines,
        functions,
    }
}

fn collect_logical_lines(content: &str) -> Vec<(usize, &str)> {
    let mut logical_lines = Vec::new();
    let mut continuation_start: Option<usize> = None;
    let mut continuation_line_no: Option<usize> = None;
    let mut offset = 0usize;
    let mut heredoc_terminator: Option<String> = None;

    for (index, raw) in content.lines().enumerate() {
        let line_no = index + 1;
        let line_start = offset;
        let line_end = line_start + raw.len();
        offset = line_end + 1;

        if let Some(terminator) = heredoc_terminator.as_deref() {
            if raw.trim() == terminator {
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
            heredoc_terminator = Some(terminator.to_owned());
        }
        logical_lines.push((continuation_line_no.unwrap_or(line_no), logical));
        continuation_start = None;
        continuation_line_no = None;
    }

    if let Some(start) = continuation_start
        && start < content.len()
    {
        logical_lines.push((continuation_line_no.unwrap_or(1), &content[start..]));
    }

    logical_lines
}

fn parse_executable_line(raw: &str, line_no: usize) -> Option<ExecutableLine<'_>> {
    let trimmed = strip_inline_comment(raw).trim();
    if trimmed.is_empty() || trimmed.starts_with('#') || is_shell_control_line(trimmed) {
        return None;
    }
    if is_constant_condition_control_line(trimmed) {
        return None;
    }
    if let Some(inner_command) = assignment_command_substitution(trimmed) {
        let softened_by = detect_fail_open_wrapper(inner_command);
        let command_text = extract_command_segment(inner_command);
        let command_name = leading_command_name(command_text)?;

        return Some(ExecutableLine {
            line_no,
            raw,
            command_text,
            command_name,
            softened_by,
            is_dispatcher_syntax: is_dispatcher_command(command_text),
            is_exit_zero: command_name == "exit" && argument_starts_with_zero(command_text),
        });
    }
    if is_standalone_assignment(trimmed) {
        return None;
    }

    let softened_by = detect_fail_open_wrapper(trimmed);
    let command_text = extract_command_segment(trimmed);
    let command_name = leading_command_name(command_text)?;

    Some(ExecutableLine {
        line_no,
        raw,
        command_text,
        command_name,
        softened_by,
        is_dispatcher_syntax: is_dispatcher_command(command_text),
        is_exit_zero: command_name == "exit" && argument_starts_with_zero(command_text),
    })
}

fn single_line_constant_if_taken_branch(raw: &str) -> Option<Option<&str>> {
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

fn extract_command_segment(line: &str) -> &str {
    let mut segment = strip_inline_comment(line).trim();

    if let Some(stripped) = segment.strip_prefix("if ") {
        segment = stripped.trim();
    }
    if let Some(stripped) = segment.strip_prefix('!') {
        segment = stripped.trim();
    }
    segment = segment.strip_suffix("; then").unwrap_or(segment).trim();
    segment = segment.strip_suffix("then").unwrap_or(segment).trim();

    if let Some((_, rhs)) = segment.rsplit_once("&&") {
        segment = rhs.trim();
    }

    segment = segment
        .trim_start_matches('(')
        .trim_end_matches(')')
        .trim_end_matches(';')
        .trim();

    if let Some((lhs, _)) = segment.split_once("||") {
        segment = lhs.trim();
    }

    segment
}

fn heredoc_delimiter(line: &str) -> Option<&str> {
    let marker = line.find("<<")?;
    let suffix = line.get(marker + 2..)?.trim_start();
    let delimiter = suffix
        .strip_prefix('\'')
        .and_then(|rest| rest.split_once('\'').map(|(token, _)| token))
        .or_else(|| {
            suffix
                .strip_prefix('"')
                .and_then(|rest| rest.split_once('"').map(|(token, _)| token))
        })
        .or_else(|| suffix.split_whitespace().next())?;
    if delimiter.is_empty() {
        None
    } else {
        Some(delimiter)
    }
}

fn strip_inline_comment(line: &str) -> &str {
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut prev_was_whitespace = true;

    for (idx, ch) in line.char_indices() {
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '#' if !single_quoted && !double_quoted && prev_was_whitespace => {
                return &line[..idx];
            }
            _ => {}
        }
        prev_was_whitespace = ch.is_whitespace();
    }

    line
}

fn leading_command_name(command_text: &str) -> Option<&str> {
    let token = command_text.split_whitespace().next()?;
    let token = token.trim_matches(|c: char| c == '(' || c == ')' || c == ';');
    if token.is_empty() { None } else { Some(token) }
}

fn detect_fail_open_wrapper(line: &str) -> Option<FailOpenWrapper<'_>> {
    let (_, rhs) = line.split_once("||")?;
    let rhs = rhs.trim();
    if rhs == "true" || rhs.starts_with("true;") {
        return Some(FailOpenWrapper::True);
    }
    if rhs == ":" || rhs.starts_with(":;") {
        return Some(FailOpenWrapper::NoOp);
    }
    if rhs.starts_with("echo ") {
        return Some(FailOpenWrapper::Echo(rhs));
    }
    None
}

fn is_dispatcher_command(command_text: &str) -> bool {
    command_text.starts_with("source ")
        || command_text.starts_with(". ")
        || command_text.contains("run-parts")
}

fn argument_starts_with_zero(command_text: &str) -> bool {
    let mut parts = command_text.split_whitespace();
    let _ = parts.next();
    matches!(parts.next(), Some("0" | "0;"))
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
    let Some((name, _value)) = line.split_once('=') else {
        return false;
    };
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !has_unquoted_whitespace(strip_assignment_prefix(line))
}

fn assignment_command_substitution(line: &str) -> Option<&str> {
    let (name, value) = line.split_once('=')?;
    let name = strip_assignment_prefix(name.trim());
    let mut chars = name.chars();
    let first = chars.next()?;
    if !(first.is_ascii_alphabetic() || first == '_')
        || !chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return None;
    }

    let mut value = value.trim();
    if value.starts_with('"') && value.ends_with('"') {
        value = &value[1..value.len() - 1];
    }
    let inner = value.strip_prefix("$(")?.strip_suffix(')')?.trim();
    if inner.is_empty() { None } else { Some(inner) }
}

fn has_unquoted_whitespace(line: &str) -> bool {
    let mut single_quoted = false;
    let mut double_quoted = false;

    for ch in line.chars() {
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            _ if ch.is_whitespace() && !single_quoted && !double_quoted => return true,
            _ => {}
        }
    }

    false
}

fn strip_assignment_prefix(name: &str) -> &str {
    name.strip_prefix("export ")
        .or_else(|| name.strip_prefix("local "))
        .or_else(|| name.strip_prefix("declare "))
        .or_else(|| name.strip_prefix("readonly "))
        .or_else(|| name.strip_prefix("typeset "))
        .unwrap_or(name)
        .trim()
}

fn is_function_definition_line(line: &str) -> bool {
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

fn function_definition_name(line: &str) -> Option<String> {
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

fn initial_function_body_fragment(line: &str) -> String {
    let Some((_, rest)) = line.split_once('{') else {
        return String::new();
    };
    trim_trailing_function_closer(rest.trim_start())
}

fn append_function_body_line(function: &mut ShellFunction, raw: &str) {
    function.body.push_str(raw);
    function.body.push('\n');
}

fn trim_trailing_function_closer(body: &str) -> String {
    let trimmed = body.trim_end();
    if trimmed == "}" {
        return String::new();
    }
    let stripped = trimmed
        .rsplit_once('}')
        .map_or(trimmed, |(before, _)| before)
        .trim_end();
    if stripped.is_empty() {
        String::new()
    } else {
        format!("{stripped}\n")
    }
}

fn inline_command_after_function_definition(line: &str) -> Option<&str> {
    let (_, tail) = line.rsplit_once('}')?;
    let tail = tail.trim_start_matches(|c: char| c == ';' || c.is_whitespace());
    (!tail.is_empty()).then_some(tail)
}

fn function_scope_depth_after_definition(line: &str) -> usize {
    brace_delta(line).max(0) as usize
}

fn update_function_scope_depth(depth: usize, line: &str) -> usize {
    depth.saturating_add_signed(brace_delta(line))
}

fn starts_dead_if_scope(line: &str) -> bool {
    let Some(condition) = line.strip_prefix("if ").and_then(|rest| {
        rest.split_once("then")
            .map(|(condition, _)| condition.trim())
    }) else {
        return false;
    };

    matches!(constant_shell_status(condition), Some(false))
}

fn dead_if_scope_depth_after_start(line: &str) -> usize {
    if line.contains("fi") { 0 } else { 1 }
}

fn update_dead_if_depth(depth: usize, line: &str) -> usize {
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

fn starts_live_true_if_scope(line: &str) -> bool {
    let Some(condition) = line.strip_prefix("if ").and_then(|rest| {
        rest.split_once("then")
            .map(|(condition, _)| condition.trim())
    }) else {
        return false;
    };

    matches!(constant_shell_status(condition), Some(true))
}

fn starts_dead_alternate_if_branch(line: &str) -> bool {
    let trimmed = strip_inline_comment(line).trim();
    trimmed == "else" || (trimmed.starts_with("elif ") && trimmed.contains("then"))
}

fn update_dead_else_depth(depth: usize, line: &str) -> usize {
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

fn starts_dead_loop_scope(line: &str) -> bool {
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

fn dead_loop_scope_depth_after_start(line: &str) -> usize {
    if line.contains("done") { 0 } else { 1 }
}

fn update_dead_loop_depth(depth: usize, line: &str) -> usize {
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

fn is_fi_line(line: &str) -> bool {
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

fn constant_shell_status(segment: &str) -> Option<bool> {
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

#[cfg(test)]
#[path = "hook_shell_tests/mod.rs"]
mod hook_shell_tests;
