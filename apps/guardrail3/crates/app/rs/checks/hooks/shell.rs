#![allow(dead_code)]

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
}

#[must_use]
pub fn parse_script(content: &str) -> ParsedShellScript<'_> {
    let mut shebang = None;
    let mut executable_lines = Vec::new();

    for (index, raw) in content.lines().enumerate() {
        let line_no = index + 1;
        let trimmed = raw.trim();
        if line_no == 1 && trimmed.starts_with("#!") {
            shebang = Some(trimmed);
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
    }
}

fn parse_executable_line(raw: &str, line_no: usize) -> Option<ExecutableLine<'_>> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') || is_shell_control_line(trimmed) {
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

fn extract_command_segment(line: &str) -> &str {
    let mut segment = line.trim();

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

fn is_standalone_assignment(line: &str) -> bool {
    if line.contains(char::is_whitespace) {
        return false;
    }
    let Some((name, _value)) = line.split_once('=') else {
        return false;
    };
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn assignment_command_substitution(line: &str) -> Option<&str> {
    let (name, value) = line.split_once('=')?;
    let mut chars = name.chars();
    let first = chars.next()?;
    if !(first.is_ascii_alphabetic() || first == '_')
        || !chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return None;
    }

    let value = value.trim();
    let inner = value.strip_prefix("$(")?.strip_suffix(')')?.trim();
    if inner.is_empty() { None } else { Some(inner) }
}

#[cfg(test)]
#[path = "shell_tests.rs"]
mod tests;
