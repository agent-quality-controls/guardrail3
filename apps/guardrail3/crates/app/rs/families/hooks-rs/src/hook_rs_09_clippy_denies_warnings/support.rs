use super::{EnvState, LintEffect};

pub(super) fn cargo_clippy_denies_warnings<'a, I>(
    parts: &mut std::iter::Peekable<I>,
    env_state: &EnvState,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    if matches!(parts.peek(), Some(token) if token.starts_with('+')) {
        let _ = parts.next();
    }

    while let Some(token) = parts.peek().copied() {
        if !token.starts_with('-') {
            break;
        }

        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if let Some((flag_name, _)) = flag.split_once('=')
            && cargo_global_flag_takes_value(flag_name)
        {
            continue;
        }
        if matches!(flag.strip_prefix("-j"), Some(value) if !value.is_empty()) {
            continue;
        }
        if cargo_global_flag_takes_value(flag) {
            let _ = parts.next();
        }
    }

    if parts.next() != Some("clippy") {
        return false;
    }

    let mut combined_tokens = env_state
        .rustflags
        .as_deref()
        .map(rustflags_tokens)
        .unwrap_or_default();

    while let Some(token) = parts.next() {
        if is_help_or_version_flag(token) {
            return false;
        }
        if token == "--" {
            combined_tokens.extend(parts.by_ref().map(str::to_owned));
            break;
        }
    }

    let token_refs: Vec<_> = combined_tokens.iter().map(String::as_str).collect();
    let effect = lint_effect_from_tokens(&token_refs);
    effect.denied && !effect.softened
}

fn rustflags_tokens(value: &str) -> Vec<String> {
    shell_words(value)
}

fn lint_effect_from_tokens(tokens: &[&str]) -> LintEffect {
    let mut effect = LintEffect::default();
    let mut warnings_level = None;
    let mut i = 0usize;

    while i < tokens.len() {
        let token = tokens[i];
        if let Some(level) = split_warning_level(token, tokens.get(i + 1).copied()) {
            warnings_level = Some(level);
            i += 1;
        } else if split_force_warn(token, tokens.get(i + 1).copied()) {
            effect.softened = true;
            i += 1;
        } else if soften_from_split_cap_lints(token, tokens.get(i + 1).copied()) {
            effect.softened = true;
            i += 1;
        } else if let Some(level) = inline_warning_level(token) {
            warnings_level = Some(level);
        } else if inline_force_warn(token) || soften_from_inline_cap_lints(token) {
            effect.softened = true;
        }
        i += 1;
    }

    match warnings_level {
        Some("deny" | "forbid") => effect.denied = true,
        Some("allow" | "warn") => effect.softened = true,
        _ => {}
    }

    effect
}

fn split_warning_level(token: &str, next: Option<&str>) -> Option<&'static str> {
    let level = match token {
        "-D" | "--deny" => "deny",
        "-A" | "--allow" => "allow",
        "-W" | "--warn" => "warn",
        "-F" | "--forbid" => "forbid",
        _ => return None,
    };
    (next == Some("warnings")).then_some(level)
}

fn split_force_warn(token: &str, next: Option<&str>) -> bool {
    token == "--force-warn" && next == Some("warnings")
}

fn soften_from_split_cap_lints(token: &str, next: Option<&str>) -> bool {
    token == "--cap-lints" && next.is_some_and(|value| !matches!(value, "deny" | "forbid"))
}

fn inline_warning_level(token: &str) -> Option<&'static str> {
    match token {
        "-Dwarnings" | "--deny=warnings" => Some("deny"),
        "-Awarnings" | "--allow=warnings" => Some("allow"),
        "-Wwarnings" | "--warn=warnings" => Some("warn"),
        "-Fwarnings" | "--forbid=warnings" => Some("forbid"),
        _ => None,
    }
}

fn inline_force_warn(token: &str) -> bool {
    token == "--force-warn=warnings"
}

fn soften_from_inline_cap_lints(token: &str) -> bool {
    token
        .strip_prefix("--cap-lints=")
        .is_some_and(|value| !matches!(value, "deny" | "forbid"))
}

pub(super) fn apply_inline_assignment(token: &str, env_state: &mut EnvState) {
    let Some((name, value)) = token.split_once('=') else {
        return;
    };
    if name == "RUSTFLAGS" {
        env_state.rustflags = Some(value.to_owned());
    }
}

pub(super) fn apply_export_assignments<'a, I>(parts: &mut I, env_state: &mut EnvState)
where
    I: Iterator<Item = &'a str>,
{
    for token in parts {
        if let Some((name, value)) = token.split_once('=')
            && name == "RUSTFLAGS"
        {
            env_state.rustflags = Some(value.to_owned());
        }
    }
}

pub(super) fn apply_unset_arguments<'a, I>(parts: &mut I, env_state: &mut EnvState)
where
    I: Iterator<Item = &'a str>,
{
    for token in parts {
        if token.starts_with('-') {
            continue;
        }
        if token == "RUSTFLAGS" {
            env_state.rustflags = None;
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

pub(super) fn shell_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "-o" | "-O" | "--init-file" | "--rcfile")
}

pub(super) fn env_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "-u" | "--unset" | "-C" | "--chdir" | "-S" | "--split-string"
    )
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
    let chars: Vec<(usize, char)> = line.char_indices().collect();
    let mut i = 0usize;

    while i < chars.len() {
        let (idx, ch) = chars[i];
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
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
