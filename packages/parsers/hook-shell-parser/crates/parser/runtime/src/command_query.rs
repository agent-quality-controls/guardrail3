use super::{ParsedShellScript, parse_script};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedCommand {
    pub line_no: usize,
    pub command_text: String,
    pub command_path: String,
    pub command_name: String,
    pub tokens: Vec<String>,
}

impl ResolvedCommand {
    #[must_use]
    pub fn line_no(&self) -> usize {
        self.line_no
    }

    #[must_use]
    pub fn command_text(&self) -> &str {
        &self.command_text
    }

    #[must_use]
    pub fn command_path(&self) -> &str {
        &self.command_path
    }

    #[must_use]
    pub fn command_name(&self) -> &str {
        &self.command_name
    }

    #[must_use]
    pub fn tokens(&self) -> &[String] {
        &self.tokens
    }

    #[must_use]
    pub fn args(&self) -> &[String] {
        self.tokens.get(1..).unwrap_or(&[])
    }

    #[must_use]
    pub fn path_qualified(&self) -> bool {
        self.command_path.contains('/')
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommandSegment {
    text: String,
    operator_before: Option<&'static str>,
    operator_after: Option<&'static str>,
}

#[derive(Debug, Clone)]
struct TokenCursor<'a> {
    tokens: &'a [String],
    index: usize,
}

impl<'a> TokenCursor<'a> {
    fn new(tokens: &'a [String]) -> Self {
        Self { tokens, index: 0 }
    }

    fn peek(&self) -> Option<&'a str> {
        self.tokens.get(self.index).map(String::as_str)
    }

    fn next(&mut self) -> Option<&'a str> {
        let token = self.peek()?;
        self.index += 1;
        Some(token)
    }

    fn remaining(&self) -> &'a [String] {
        self.tokens.get(self.index..).unwrap_or(&[])
    }
}

#[must_use]
pub fn any_resolved_command(
    parsed: &ParsedShellScript,
    predicate: fn(&ResolvedCommand) -> bool,
) -> bool {
    any_resolved_command_with_mode(parsed, predicate, false)
}

#[must_use]
pub fn any_resolved_command_on_line(
    parsed: &ParsedShellScript,
    raw: &str,
    line_no: usize,
    predicate: fn(&ResolvedCommand) -> bool,
) -> bool {
    let mut visiting = Vec::new();
    line_matches(raw, parsed, &mut visiting, predicate, line_no)
}

#[must_use]
pub fn any_resolved_command_relaxed(
    parsed: &ParsedShellScript,
    predicate: fn(&ResolvedCommand) -> bool,
) -> bool {
    any_resolved_command_with_mode(parsed, predicate, true)
}

fn any_resolved_command_with_mode(
    parsed: &ParsedShellScript,
    predicate: fn(&ResolvedCommand) -> bool,
    allow_detached: bool,
) -> bool {
    let mut visiting = Vec::new();
    parsed.executable_lines().iter().any(|line| {
        line_matches_with_mode(
            line.raw(),
            parsed,
            &mut visiting,
            predicate,
            line.line_no(),
            allow_detached,
        )
    })
}

fn line_matches(
    raw: &str,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: fn(&ResolvedCommand) -> bool,
    line_no: usize,
) -> bool {
    line_matches_with_mode(raw, root, visiting, predicate, line_no, false)
}

fn line_matches_with_mode(
    raw: &str,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: fn(&ResolvedCommand) -> bool,
    line_no: usize,
    allow_detached: bool,
) -> bool {
    let segments = split_command_segments(raw);
    if segments.is_empty() {
        return segment_matches(raw, root, visiting, predicate, line_no);
    }

    let mut prefix_status = None;
    for segment in segments {
        let reachable = match (segment.operator_before, prefix_status) {
            (Some("&&"), Some(true)) => true,
            (Some("&&"), Some(false)) => false,
            (Some("||"), Some(true)) => false,
            (Some("||"), Some(false)) => true,
            _ => true,
        };

        let detached = matches!(segment.operator_after, Some("&" | "|"));
        if reachable && (allow_detached || !detached) {
            if segment_matches(&segment.text, root, visiting, predicate, line_no) {
                return true;
            }
            for substitution in extract_command_substitutions(&segment.text) {
                if line_matches_with_mode(
                    &substitution,
                    root,
                    visiting,
                    predicate,
                    line_no,
                    allow_detached,
                ) {
                    return true;
                }
            }
        }

        if reachable {
            if is_terminal_exit(&segment.text) {
                break;
            }
            prefix_status = constant_exit_status(&segment.text);
        }
    }

    false
}

fn segment_matches(
    segment: &str,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: fn(&ResolvedCommand) -> bool,
    line_no: usize,
) -> bool {
    token_sequence_matches(shell_words(segment), root, visiting, predicate, line_no)
}

fn token_sequence_matches(
    tokens: Vec<String>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: fn(&ResolvedCommand) -> bool,
    line_no: usize,
) -> bool {
    let mut cursor = TokenCursor::new(&tokens);

    while cursor.peek() == Some("!") {
        let _ = cursor.next();
    }

    while cursor.peek().is_some_and(looks_like_env_assignment) {
        let _ = cursor.next();
    }

    let Some(first) = cursor.next() else {
        return false;
    };

    dispatch_token(first, &mut cursor, root, visiting, predicate, line_no)
}

fn dispatch_token(
    token: &str,
    cursor: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: fn(&ResolvedCommand) -> bool,
    line_no: usize,
) -> bool {
    match normalize_command_token(token) {
        "env" => env_wrapper_matches(cursor, root, visiting, predicate, line_no),
        "sh" | "bash" => shell_wrapper_matches(cursor, root, visiting, predicate, line_no),
        "command" => command_wrapper_matches(cursor, root, visiting, predicate, line_no),
        "exec" => exec_wrapper_matches(cursor, root, visiting, predicate, line_no),
        command_name if !token.contains('/') => {
            if function_defined_before(command_name, root, line_no) {
                return called_function_matches(command_name, root, visiting, predicate, line_no);
            }
            predicate(&resolved_command(token, cursor, line_no))
        }
        _ => predicate(&resolved_command(token, cursor, line_no)),
    }
}

fn dispatch_external_token(
    token: &str,
    cursor: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: fn(&ResolvedCommand) -> bool,
    line_no: usize,
) -> bool {
    match normalize_command_token(token) {
        "env" => env_wrapper_matches(cursor, root, visiting, predicate, line_no),
        "sh" | "bash" => shell_wrapper_matches(cursor, root, visiting, predicate, line_no),
        "command" => command_wrapper_matches(cursor, root, visiting, predicate, line_no),
        "exec" => exec_wrapper_matches(cursor, root, visiting, predicate, line_no),
        _ => predicate(&resolved_command(token, cursor, line_no)),
    }
}

fn function_defined_before(
    command_name: &str,
    root: &ParsedShellScript,
    line_no: usize,
) -> bool {
    root.functions()
        .iter()
        .any(|function| function.name() == command_name && function.line_no() <= line_no)
}

fn called_function_matches(
    command_name: &str,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: fn(&ResolvedCommand) -> bool,
    line_no: usize,
) -> bool {
    let Some(function) = root
        .functions()
        .iter()
        .find(|function| function.name() == command_name && function.line_no() <= line_no)
    else {
        return false;
    };
    if visiting.iter().any(|name| name == function.name()) {
        return false;
    }

    visiting.push(function.name().to_owned());
    let body = parse_script(function.body());
    let found = body.executable_lines().iter().any(|line| {
        line_matches(line.raw(), &body, visiting, predicate, line.line_no())
            || line_matches(line.raw(), root, visiting, predicate, line.line_no())
    });
    let _ = visiting.pop();
    found
}

fn resolved_command(token: &str, cursor: &TokenCursor<'_>, line_no: usize) -> ResolvedCommand {
    let mut tokens = vec![token.to_owned()];
    tokens.extend(cursor.remaining().iter().cloned());
    let command_text = tokens.join(" ");

    ResolvedCommand {
        line_no,
        command_text,
        command_path: token.to_owned(),
        command_name: normalize_command_token(token).to_owned(),
        tokens,
    }
}

fn env_wrapper_matches(
    cursor: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: fn(&ResolvedCommand) -> bool,
    line_no: usize,
) -> bool {
    let mut split_string = None;

    while cursor.peek().is_some_and(|token| token.starts_with('-')) {
        let flag = cursor.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if flag == "--" {
            break;
        }
        if let Some((flag_name, value)) = flag.split_once('=')
            && env_flag_takes_value(flag_name)
        {
            if matches!(flag_name, "-S" | "--split-string") {
                split_string = Some(value.to_owned());
            }
            continue;
        }
        if env_flag_without_value(flag) {
            continue;
        }
        if env_flag_takes_value(flag) {
            let value = cursor.next().unwrap_or_default();
            if matches!(flag, "-S" | "--split-string") {
                split_string = Some(value.to_owned());
            }
            continue;
        }

        return false;
    }

    while cursor.peek().is_some_and(looks_like_env_assignment) {
        let _ = cursor.next();
    }

    if let Some(script) = split_string {
        let mut split_tokens =
            if !cursor.remaining().is_empty() && looks_like_env_assignment(&script) {
                vec![script]
            } else {
                shell_words(&script)
            };
        split_tokens.extend(cursor.remaining().iter().cloned());
        return token_sequence_matches(split_tokens, root, visiting, predicate, line_no);
    }

    let Some(next) = cursor.next() else {
        return false;
    };

    dispatch_external_token(next, cursor, root, visiting, predicate, line_no)
}

fn shell_wrapper_matches(
    cursor: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: fn(&ResolvedCommand) -> bool,
    line_no: usize,
) -> bool {
    let mut script = None;

    while let Some(token) = cursor.peek() {
        if !token.starts_with('-') {
            break;
        }

        let flag = cursor.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }

        if let Some(value) = shell_inline_script(flag) {
            script = Some(value.to_owned());
            continue;
        }

        if shell_cluster_uses_next_script(flag) {
            script = Some(cursor.next().unwrap_or_default().to_owned());
            continue;
        }

        if let Some((flag_name, value)) = flag.split_once('=')
            && shell_flag_takes_value(flag_name)
        {
            if flag_name == "-c" {
                script = Some(value.to_owned());
            }
            continue;
        }

        if shell_flag_takes_value(flag) {
            let value = cursor.next().unwrap_or_default();
            if flag == "-c" {
                script = Some(value.to_owned());
            }
            continue;
        }

        return false;
    }

    if let Some(script) = script {
        return line_matches(&script, root, visiting, predicate, line_no);
    }

    let Some(next) = cursor.next() else {
        return false;
    };

    dispatch_external_token(next, cursor, root, visiting, predicate, line_no)
}

fn shell_inline_script(flag: &str) -> Option<&str> {
    if !flag.starts_with('-') || flag.starts_with("--") {
        return None;
    }

    let short = &flag[1..];
    let c_pos = short.find('c')?;
    let attached = short.get(c_pos + 1..)?;
    (!attached.is_empty()).then_some(attached)
}

fn shell_cluster_uses_next_script(flag: &str) -> bool {
    if !flag.starts_with('-') || flag.starts_with("--") {
        return false;
    }

    let short = &flag[1..];
    short.len() > 1 && short.ends_with('c')
}

fn command_wrapper_matches(
    cursor: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: fn(&ResolvedCommand) -> bool,
    line_no: usize,
) -> bool {
    while cursor.peek().is_some_and(|token| token.starts_with('-')) {
        let flag = cursor.next().unwrap_or_default();
        if is_help_or_version_flag(flag) || matches!(flag, "-v" | "-V") {
            return false;
        }
        if flag == "--" {
            break;
        }
        if flag != "-p" {
            return false;
        }
    }

    let Some(next) = cursor.next() else {
        return false;
    };

    dispatch_external_token(next, cursor, root, visiting, predicate, line_no)
}

fn exec_wrapper_matches(
    cursor: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: fn(&ResolvedCommand) -> bool,
    line_no: usize,
) -> bool {
    while cursor.peek().is_some_and(|token| token.starts_with('-')) {
        let flag = cursor.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if flag == "--" {
            break;
        }
        if exec_flag_takes_value(flag) {
            let _ = cursor.next();
            continue;
        }

        return false;
    }

    let Some(next) = cursor.next() else {
        return false;
    };

    dispatch_token(next, cursor, root, visiting, predicate, line_no)
}

fn split_command_segments(raw: &str) -> Vec<CommandSegment> {
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

    if !segment.contains("$(") {
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
            '$' if !single_quoted && !is_escaped(chars.as_slice(), i) => {
                if chars.get(i + 1).is_some_and(|(_, next)| *next == '(') {
                    command_substitution_depth += 1;
                    i += 1;
                }
            }
            ')' if !single_quoted && command_substitution_depth > 0 => {
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

fn constant_exit_status(segment: &str) -> Option<bool> {
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

fn is_terminal_exit(segment: &str) -> bool {
    let mut segment = segment.trim().trim_end_matches(';').trim();
    while let Some(stripped) = segment.strip_prefix('!') {
        segment = stripped.trim();
    }
    segment = segment.trim_matches(|c: char| c == '(' || c == ')' || c == '{' || c == '}');
    segment.starts_with("exit ")
}

fn shell_words(command_text: &str) -> Vec<String> {
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
                    while matches!(chars.peek(), Some(next) if next.is_whitespace()) {
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

fn extract_command_substitutions(line: &str) -> Vec<String> {
    let mut substitutions = Vec::new();
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut depth = 0usize;
    let mut start = None;
    let chars: Vec<(usize, char)> = line.char_indices().collect();
    let mut i = 0usize;

    while i < chars.len() {
        let (idx, ch) = chars[i];
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '$' if !single_quoted && !is_escaped(chars.as_slice(), i) => {
                if chars.get(i + 1).is_some_and(|(_, next)| *next == '(') {
                    if depth == 0 {
                        start = chars.get(i + 2).map(|(next_idx, _)| *next_idx);
                    }
                    depth += 1;
                    i += 1;
                }
            }
            ')' if !single_quoted && depth > 0 => {
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

fn is_escaped(chars: &[(usize, char)], index: usize) -> bool {
    let mut backslashes = 0usize;
    let mut cursor = index;

    while cursor > 0 {
        cursor -= 1;
        if chars[cursor].1 == '\\' {
            backslashes += 1;
        } else {
            break;
        }
    }

    backslashes % 2 == 1
}

fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

fn env_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "-u" | "--unset" | "-C" | "--chdir" | "-S" | "--split-string"
    )
}

fn env_flag_without_value(flag: &str) -> bool {
    matches!(flag, "-i" | "--ignore-environment")
}

fn shell_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "-c" | "-o" | "-O" | "--init-file" | "--rcfile")
}

fn exec_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "-a")
}

fn normalize_command_token(token: &str) -> &str {
    token.rsplit('/').next().unwrap_or(token)
}

fn looks_like_env_assignment(token: &str) -> bool {
    let Some((name, _)) = token.split_once('=') else {
        return false;
    };
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}
