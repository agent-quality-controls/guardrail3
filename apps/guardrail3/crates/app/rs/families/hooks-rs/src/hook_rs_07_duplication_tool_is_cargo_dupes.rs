use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-07";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let has_cargo_dupes = input
        .parsed
        .executable_lines
        .iter()
        .any(|line| line_contains_command(line.raw, line.command_text, is_cargo_dupes_command));
    let has_jscpd = input
        .parsed
        .executable_lines
        .iter()
        .any(|line| line_contains_command(line.raw, line.command_text, is_jscpd_command));

    if has_jscpd && !has_cargo_dupes {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "wrong Rust duplication tool".to_owned(),
            message: "Hook uses jscpd for Rust duplication checks instead of cargo-dupes."
                .to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    } else if has_cargo_dupes {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "cargo-dupes selected for Rust duplication checks".to_owned(),
                message: "Hook uses cargo-dupes for Rust duplication checks.".to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "Rust duplication tool missing".to_owned(),
            message: "Hook does not show a Rust duplication-check command.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

fn line_contains_command(raw: &str, command_text: &str, predicate: fn(&str) -> bool) -> bool {
    let mut segments = split_command_segments(raw);
    let recovered_substitutions: Vec<String> = segments
        .iter()
        .filter_map(|segment| inline_command_substitution(segment))
        .map(str::to_owned)
        .collect();
    if !segments.iter().any(|segment| segment == command_text) {
        segments.push(command_text.to_owned());
    }
    segments.extend(recovered_substitutions);
    segments.iter().any(|segment| predicate(segment))
}

fn split_command_segments(raw: &str) -> Vec<String> {
    let mut line = raw.trim();

    if let Some(stripped) = line.strip_prefix("if ") {
        line = stripped.trim();
    }
    if let Some(stripped) = line.strip_prefix('!') {
        line = stripped.trim();
    }
    line = line.strip_suffix("; then").unwrap_or(line).trim();
    line = line.strip_suffix("then").unwrap_or(line).trim();

    split_unquoted_commands(line)
        .into_iter()
        .map(|segment| {
            segment
                .trim_matches(|c: char| {
                    c == '(' || c == ')' || c == '{' || c == '}' || c == ';' || c == '&' || c == '|'
                })
                .trim()
                .to_owned()
        })
        .filter(|segment| !segment.is_empty())
        .collect()
}

fn split_unquoted_commands(line: &str) -> Vec<&str> {
    let mut segments = Vec::new();
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut start = 0usize;
    let chars: Vec<(usize, char)> = line.char_indices().collect();
    let mut i = 0usize;

    while i < chars.len() {
        let (idx, ch) = chars[i];
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            ';' if !single_quoted && !double_quoted => {
                if start < idx {
                    segments.push(line[start..idx].trim());
                }
                start = idx + ch.len_utf8();
            }
            '&' if !single_quoted && !double_quoted => {
                let next_is_ampersand = chars.get(i + 1).is_some_and(|(_, next)| *next == '&');
                if next_is_ampersand {
                    if start < idx {
                        segments.push(line[start..idx].trim());
                    }
                    let next_idx = chars[i + 1].0;
                    start = next_idx + 1;
                    i += 1;
                }
            }
            '|' if !single_quoted && !double_quoted => {
                let next_is_pipe = chars.get(i + 1).is_some_and(|(_, next)| *next == '|');
                if next_is_pipe {
                    if start < idx {
                        segments.push(line[start..idx].trim());
                    }
                    let next_idx = chars[i + 1].0;
                    start = next_idx + 1;
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    if start < line.len() {
        segments.push(line[start..].trim());
    }

    segments
}

fn is_cargo_dupes_command(command_text: &str) -> bool {
    let tokens = shell_words(command_text);
    let mut parts = tokens.iter().map(String::as_str).peekable();

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    let Some(first) = parts.next() else {
        return false;
    };

    let first = normalize_command_token(first);
    if first == "env" {
        let Some(next) = unwrap_env_command(&mut parts) else {
            return false;
        };
        if next.contains(char::is_whitespace) {
            return is_cargo_dupes_command(next);
        }
        return match normalize_command_token(next) {
            "cargo" => is_cargo_dupes_invocation(&mut parts),
            "cargo-dupes" => !parts.any(is_help_or_version_flag),
            _ => false,
        };
    }

    match first {
        "cargo" => is_cargo_dupes_invocation(&mut parts),
        "cargo-dupes" => !parts.any(is_help_or_version_flag),
        _ => false,
    }
}

fn is_jscpd_command(command_text: &str) -> bool {
    let tokens = shell_words(command_text);
    let mut parts = tokens.iter().map(String::as_str).peekable();

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    let Some(first) = parts.next() else {
        return false;
    };

    let first = normalize_command_token(first);
    if first == "env" {
        let Some(next) = unwrap_env_command(&mut parts) else {
            return false;
        };
        if next.contains(char::is_whitespace) {
            return is_jscpd_command(next);
        }
        return normalize_command_token(next) == "jscpd" && !parts.any(is_help_or_version_flag);
    }

    first == "jscpd" && !parts.any(is_help_or_version_flag)
}

fn unwrap_env_command<'a, I>(parts: &mut std::iter::Peekable<I>) -> Option<&'a str>
where
    I: Iterator<Item = &'a str>,
{
    while let Some(token) = parts.peek().copied() {
        if looks_like_env_assignment(token) {
            let _ = parts.next();
            continue;
        }
        if !token.starts_with('-') {
            break;
        }

        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return None;
        }
        if matches!(flag, "-S" | "--split-string") {
            let split_command = parts.next()?;
            if split_string_is_assignment_only(split_command) {
                continue;
            }
            return Some(split_command);
        }
        if env_flag_takes_value(flag) {
            let _ = parts.next();
        }
    }

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    parts.next()
}

fn split_string_is_assignment_only(payload: &str) -> bool {
    let tokens = shell_words(payload);
    let Some(first) = tokens.first() else {
        return false;
    };
    looks_like_env_assignment(first)
        && !tokens.iter().any(|token| {
            matches!(
                normalize_command_token(token),
                "cargo" | "cargo-dupes" | "jscpd" | "env"
            )
        })
}

fn inline_command_substitution(segment: &str) -> Option<&str> {
    let start = segment.find("$(")?;
    let inner = segment.get(start + 2..)?;
    let inner = inner.strip_suffix(')')?.trim();
    if inner.is_empty() { None } else { Some(inner) }
}

fn env_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "-u" | "--unset" | "-C" | "--chdir" | "-S" | "--split-string"
    )
}

fn is_cargo_dupes_invocation<'a, I>(parts: &mut std::iter::Peekable<I>) -> bool
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
        if cargo_global_flag_takes_value(flag) {
            let _ = parts.next();
        }
    }

    parts.next() == Some("dupes") && !parts.any(is_help_or_version_flag)
}

fn cargo_global_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "--config" | "-Z" | "--manifest-path" | "--color" | "--target" | "--target-dir" | "--jobs"
    )
}

fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

fn normalize_command_token(token: &str) -> &str {
    token.rsplit('/').next().unwrap_or(token)
}

fn looks_like_env_assignment(token: &str) -> bool {
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

fn shell_words(command_text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut chars = command_text.chars().peekable();
    let mut single_quoted = false;
    let mut double_quoted = false;

    while let Some(ch) = chars.next() {
        match ch {
            '\'' if !double_quoted => {
                single_quoted = !single_quoted;
            }
            '"' if !single_quoted => {
                double_quoted = !double_quoted;
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

#[cfg(test)]
pub(super) fn run_case(content: &str) -> Vec<CheckResult> {
    let parsed = test_support::parsed_hook(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "hook_rs_07_duplication_tool_is_cargo_dupes_tests/mod.rs"]
mod hook_rs_07_duplication_tool_is_cargo_dupes_tests;
