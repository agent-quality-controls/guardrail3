use super::rule::EnvState;

#[derive(Debug, Clone)]
pub(super) struct TokenCursor<'a> {
    tokens: &'a [String],
    index: usize,
}

impl<'a> TokenCursor<'a> {
    pub(super) fn new(tokens: &'a [String]) -> Self {
        Self { tokens, index: 0 }
    }

    pub(super) fn peek(&self) -> Option<&'a str> {
        self.tokens.get(self.index).map(String::as_str)
    }

    pub(super) fn next(&mut self) -> Option<&'a str> {
        let token = self.peek()?;
        self.index += 1;
        Some(token)
    }

    pub(super) fn remaining(&self) -> &'a [String] {
        self.tokens.get(self.index..).unwrap_or(&[])
    }
}

pub(super) fn apply_inline_assignment(token: &str, env_state: &mut EnvState) {
    let Some((name, _value)) = token.split_once('=') else {
        return;
    };
    if name == "CARGO_TARGET_DIR" {
        env_state.target_dir = true;
    }
}

pub(super) fn apply_export_assignments(parts: &mut TokenCursor<'_>, env_state: &mut EnvState) {
    while let Some(token) = parts.next() {
        if let Some((name, _value)) = token.split_once('=')
            && name == "CARGO_TARGET_DIR"
        {
            env_state.target_dir = true;
        }
    }
}

pub(super) fn apply_unset_arguments(parts: &mut TokenCursor<'_>, env_state: &mut EnvState) {
    while let Some(token) = parts.next() {
        if token.starts_with('-') {
            continue;
        }
        if token == "CARGO_TARGET_DIR" {
            env_state.target_dir = false;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CommandSegment {
    pub(super) text: String,
    pub(super) operator_before: Option<&'static str>,
    pub(super) operator_after: Option<&'static str>,
}

pub(super) fn split_command_segments(raw: &str) -> Vec<CommandSegment> {
    let mut line = strip_inline_comment(raw).trim();

    if let Some(stripped) = line.strip_prefix("if ") {
        line = stripped.trim();
    }
    line = line.strip_suffix("; then").unwrap_or(line).trim();
    line = line.strip_suffix("then").unwrap_or(line).trim();

    let pieces = split_unquoted_commands(line);
    let trailing_operator = trailing_control_operator(line);
    pieces
        .iter()
        .enumerate()
        .map(|(index, (segment, operator_before))| {
            let mut segment = segment.trim();
            if let Some(stripped) = segment.strip_prefix("then ") {
                segment = stripped.trim();
            }
            segment = segment
                .trim_end_matches(|c: char| c == ';' || c == ' ')
                .trim();
            if let Some(stripped) = segment.strip_suffix(" fi") {
                segment = stripped.trim();
            }

            CommandSegment {
                text: normalize_segment_text(segment),
                operator_before: *operator_before,
                operator_after: pieces.get(index + 1).and_then(|(_, op)| *op).or_else(|| {
                    (index + 1 == pieces.len())
                        .then_some(trailing_operator)
                        .flatten()
                }),
            }
        })
        .filter(|segment| !segment.text.is_empty())
        .collect()
}

fn normalize_segment_text(segment: &str) -> String {
    let mut segment = segment
        .trim_matches(|c: char| c == '{' || c == '}' || c == ';' || c == '&' || c == '|')
        .trim();

    if segment.starts_with('(') && segment.ends_with(')') && !segment.contains("$(") {
        segment = segment.trim_start_matches('(').trim_end_matches(')').trim();
    }

    segment.to_owned()
}

fn split_unquoted_commands(line: &str) -> Vec<(&str, Option<&'static str>)> {
    let mut segments = Vec::new();
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut command_substitution_depth = 0usize;
    let mut start = 0usize;
    let mut operator_before = None;
    let chars: Vec<(usize, char)> = line.char_indices().collect();
    let mut i = 0usize;

    while i < chars.len() {
        let (idx, ch) = chars[i];
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '$' if !single_quoted && !double_quoted => {
                if chars.get(i + 1).is_some_and(|(_, next)| *next == '(') {
                    command_substitution_depth += 1;
                    i += 1;
                }
            }
            ')' if !single_quoted && !double_quoted && command_substitution_depth > 0 => {
                command_substitution_depth -= 1;
            }
            ';' if !single_quoted && !double_quoted && command_substitution_depth == 0 => {
                if start < idx {
                    segments.push((line[start..idx].trim(), operator_before));
                }
                operator_before = Some(";");
                start = idx + ch.len_utf8();
            }
            '&' if !single_quoted && !double_quoted && command_substitution_depth == 0 => {
                let next_is_ampersand = chars.get(i + 1).is_some_and(|(_, next)| *next == '&');
                if start < idx {
                    segments.push((line[start..idx].trim(), operator_before));
                }
                operator_before = Some(if next_is_ampersand { "&&" } else { "&" });
                let next_idx = if next_is_ampersand {
                    chars[i + 1].0
                } else {
                    idx
                };
                start = next_idx + 1;
                if next_is_ampersand {
                    i += 1;
                }
            }
            '|' if !single_quoted && !double_quoted && command_substitution_depth == 0 => {
                let next_is_pipe = chars.get(i + 1).is_some_and(|(_, next)| *next == '|');
                if start < idx {
                    segments.push((line[start..idx].trim(), operator_before));
                }
                operator_before = Some(if next_is_pipe { "||" } else { "|" });
                let next_idx = if next_is_pipe { chars[i + 1].0 } else { idx };
                start = next_idx + 1;
                if next_is_pipe {
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    if start < line.len() {
        segments.push((line[start..].trim(), operator_before));
    }

    segments
}

fn trailing_control_operator(line: &str) -> Option<&'static str> {
    let trimmed = line.trim_end();
    if trimmed.ends_with("&&") || trimmed.ends_with("||") {
        return None;
    }
    if trimmed.ends_with('&') {
        return Some("&");
    }
    if trimmed.ends_with('|') {
        return Some("|");
    }
    None
}

pub(super) fn constant_exit_status(segment: &str) -> Option<bool> {
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

pub(super) fn shell_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "-o" | "-O" | "--init-file" | "--rcfile")
}

pub(super) fn env_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "-u" | "--unset" | "-C" | "--chdir" | "-S" | "--split-string"
    )
}

pub(super) fn env_flag_without_value(flag: &str) -> bool {
    matches!(flag, "-i" | "--ignore-environment")
}

pub(super) fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

pub(super) fn normalize_command_token(token: &str) -> &str {
    token.rsplit('/').next().unwrap_or(token)
}

pub(super) fn looks_like_env_assignment(token: &str) -> bool {
    let Some((name, _value)) = token.split_once('=') else {
        return false;
    };
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub(super) fn shell_words(command_text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut chars = command_text.chars().peekable();
    let mut single_quoted = false;
    let mut double_quoted = false;

    while let Some(ch) = chars.next() {
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '\\' if !single_quoted && !double_quoted => {
                if matches!(chars.peek(), Some('\n')) {
                    let _ = chars.next();
                    while matches!(chars.peek(), Some(ch) if ch.is_whitespace()) {
                        let _ = chars.next();
                    }
                    continue;
                }
                current.push(ch);
            }
            '\\' if double_quoted => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ch if ch.is_whitespace() && !single_quoted && !double_quoted => {
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}

pub(super) fn extract_command_substitutions(line: &str) -> Vec<String> {
    let mut substitutions = Vec::new();
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut depth = 0usize;
    let mut start = None;
    let mut backtick_start = None;
    let chars: Vec<(usize, char)> = line.char_indices().collect();
    let mut i = 0usize;

    while i < chars.len() {
        let (idx, ch) = chars[i];
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '`' if !single_quoted && !double_quoted && depth == 0 => {
                if let Some(start_idx) = backtick_start.take() {
                    substitutions.push(line[start_idx..idx].trim().to_owned());
                } else {
                    backtick_start = chars.get(i + 1).map(|(next_idx, _)| *next_idx);
                }
            }
            '$' if !single_quoted && !double_quoted => {
                if chars.get(i + 1).is_some_and(|(_, next)| *next == '(') {
                    if depth == 0 {
                        start = chars.get(i + 2).map(|(next_idx, _)| *next_idx);
                    }
                    depth += 1;
                    i += 1;
                }
            }
            ')' if !single_quoted && !double_quoted && depth > 0 => {
                depth -= 1;
                if depth == 0
                    && let Some(start_idx) = start.take()
                {
                    substitutions.push(line[start_idx..idx].trim().to_owned());
                }
            }
            _ => {}
        }
        i += 1;
    }

    substitutions
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
