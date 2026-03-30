use guardrail3_app_rs_family_hooks_shared::hook_shell::ParsedShellScript;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CommandSegment {
    pub(crate) text: String,
    pub(crate) operator_before: Option<&'static str>,
    pub(crate) operator_after: Option<&'static str>,
}

pub(crate) fn split_command_segments(raw: &str) -> Vec<CommandSegment> {
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

pub(crate) fn constant_exit_status(segment: &str) -> Option<bool> {
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

pub(crate) fn is_terminal_exit(segment: &str) -> bool {
    let mut segment = segment.trim().trim_end_matches(';').trim();
    while let Some(stripped) = segment.strip_prefix('!') {
        segment = stripped.trim();
    }
    segment = segment.trim_matches(|c: char| c == '(' || c == ')' || c == '{' || c == '}');
    segment.starts_with("exit ")
}

pub(super) fn cargo_global_flag_takes_value(flag: &str) -> bool {
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

pub(super) fn cargo_global_flag_allowed_without_value(flag: &str) -> bool {
    matches!(
        flag,
        "-q" | "--quiet" | "-v" | "--verbose" | "--frozen" | "--locked" | "--offline"
    )
}

pub(super) fn cargo_dupes_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "--max-exact" | "--max-exact-percent")
}

pub(super) fn shell_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "-c" | "-o" | "-O")
}

pub(super) fn shell_flag_allowed_without_value(flag: &str) -> bool {
    matches!(
        flag,
        "-e" | "-i"
            | "-l"
            | "-n"
            | "-r"
            | "-s"
            | "-u"
            | "-v"
            | "-x"
            | "--login"
            | "--noprofile"
            | "--norc"
            | "--posix"
            | "--restricted"
            | "--verbose"
    )
}

pub(super) fn env_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "-C" | "--chdir" | "-S" | "--split-string" | "-u" | "--unset"
    )
}

pub(super) fn env_flag_allowed_without_value(flag: &str) -> bool {
    matches!(flag, "-i" | "--ignore-environment")
}

pub(super) fn exec_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "-a")
}

pub(super) fn exec_flag_allowed_without_value(flag: &str) -> bool {
    matches!(flag, "-c" | "-l")
}

pub(super) fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

pub(crate) fn normalize_command_token(token: &str) -> &str {
    token.rsplit('/').next().unwrap_or(token)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ShellShortFlagCluster {
    Valid { script: Option<String> },
    Invalid,
}

pub(super) fn parse_shell_short_flag_cluster<'a, I>(
    flag: &str,
    parts: &mut std::iter::Peekable<I>,
) -> Option<ShellShortFlagCluster>
where
    I: Iterator<Item = &'a str>,
{
    if flag.starts_with("--") || !flag.starts_with('-') || flag.len() <= 2 {
        return None;
    }

    let short_flags = &flag[1..];
    let mut chars = short_flags.char_indices().peekable();

    while let Some((idx, ch)) = chars.next() {
        match ch {
            'e' | 'i' | 'l' | 'n' | 'r' | 's' | 'u' | 'v' | 'x' => {}
            'c' | 'o' | 'O' => {
                let remainder = &short_flags[idx + 1..];
                if ch == 'c' {
                    if remainder.is_empty() {
                        return Some(ShellShortFlagCluster::Valid {
                            script: Some(parts.next().unwrap_or_default().to_owned()),
                        });
                    }
                    return Some(ShellShortFlagCluster::Valid {
                        script: Some(remainder.to_owned()),
                    });
                }

                if remainder.is_empty() {
                    let _ = parts.next();
                }
                return Some(ShellShortFlagCluster::Valid { script: None });
            }
            _ => return Some(ShellShortFlagCluster::Invalid),
        }
    }

    Some(ShellShortFlagCluster::Valid { script: None })
}

pub(crate) fn looks_like_env_assignment(token: &str) -> bool {
    let Some((name, _)) = token.split_once('=') else {
        return false;
    };
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte == b'_' || byte.is_ascii_alphanumeric())
}

pub(super) fn split_string_is_assignment_only(payload: &str) -> bool {
    let tokens = shell_words(payload);
    let Some(first) = tokens.first() else {
        return false;
    };
    looks_like_env_assignment(first)
        && !tokens.iter().any(|token| {
            matches!(
                normalize_command_token(token),
                "cargo" | "cargo-dupes" | "env"
            )
        })
}

pub(crate) fn token_is_shadowed_function(
    token: &str,
    command_name: &str,
    current: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    current_cutoff: usize,
    root_cutoff: usize,
) -> bool {
    if token.contains('/') {
        return false;
    }

    current
        .functions
        .iter()
        .any(|function| function.name == command_name && function.line_no <= current_cutoff)
        || root
            .functions
            .iter()
            .any(|function| function.name == command_name && function.line_no <= root_cutoff)
}

pub(crate) fn shell_words(command_text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut chars = command_text.chars().peekable();
    let mut single_quoted = false;
    let mut double_quoted = false;

    while let Some(ch) = chars.next() {
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '\\' if !single_quoted => {
                if matches!(chars.peek(), Some('\n')) {
                    let _ = chars.next();
                } else if let Some(next) = chars.next() {
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

pub(crate) fn extract_command_substitutions(line: &str) -> Vec<String> {
    let mut substitutions = Vec::new();
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut depth = 0usize;
    let chars: Vec<(usize, char)> = line.char_indices().collect();
    let mut i = 0usize;
    let mut start = None;
    let mut escaped = false;

    while i < chars.len() {
        let (idx, ch) = chars[i];

        if escaped {
            escaped = false;
            i += 1;
            continue;
        }

        match ch {
            '\\' if !single_quoted => {
                escaped = true;
            }
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '$' if !single_quoted => {
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

    for (index, ch) in line.char_indices() {
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '#' if !single_quoted && !double_quoted && prev_was_whitespace => {
                return &line[..index];
            }
            _ => {}
        }
        prev_was_whitespace = ch.is_whitespace();
    }

    line
}
