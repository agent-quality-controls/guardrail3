use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;
use guardrail3_app_rs_family_hooks_shared::hook_shell::{ParsedShellScript, parse_script};

const ID: &str = "HOOK-RS-08";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = script_contains_guardrail_step(input.parsed);

    push_presence_result(
        found,
        input.rel_path,
        "Rust guardrail validate step present",
        "Hook runs guardrail3 Rust validation on staged changes.",
        "Rust guardrail validate step missing",
        "Hook does not execute `guardrail3 ... validate --staged`.",
        results,
    );
}

pub(crate) fn script_contains_guardrail_step(parsed: &ParsedShellScript<'_>) -> bool {
    script_contains_guardrail_step_matching(parsed, is_guardrail_validate_staged_command)
}

pub(crate) fn script_contains_path_qualified_guardrail_step(
    parsed: &ParsedShellScript<'_>,
) -> bool {
    script_contains_guardrail_step_matching(
        parsed,
        is_path_qualified_guardrail_validate_staged_command,
    )
}

fn script_contains_guardrail_step_matching(
    parsed: &ParsedShellScript<'_>,
    predicate: fn(&str) -> bool,
) -> bool {
    parsed.executable_lines().iter().any(|line| {
        executable_line_or_called_function_contains_guardrail(
            line,
            parsed,
            parsed,
            &mut Vec::new(),
            predicate,
        )
    })
}

fn executable_line_or_called_function_contains_guardrail(
    line: &guardrail3_app_rs_family_hooks_shared::hook_shell::ExecutableLine<'_>,
    _parsed: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
    predicate: fn(&str) -> bool,
) -> bool {
    if line_contains_command(line.raw(), line.command_text(), predicate) {
        return true;
    }

    called_function_contains_guardrail(line.command_name(), root, visiting, predicate)
}

fn called_function_contains_guardrail(
    command_name: &str,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
    predicate: fn(&str) -> bool,
) -> bool {
    let Some(function) = root
        .functions()
        .iter()
        .find(|function| function.name() == command_name)
    else {
        return false;
    };
    if visiting.iter().any(|name| name == &function.name()) {
        return false;
    }

    visiting.push(function.name().to_owned());
    let body_parsed = parse_script(&function.body());
    let found = body_parsed.executable_lines().iter().any(|line| {
        executable_line_or_called_function_contains_guardrail(
            line,
            &body_parsed,
            root,
            visiting,
            predicate,
        )
    });
    let _ = visiting.pop();
    found
}

fn push_presence_result(
    found: bool,
    rel_path: &str,
    ok_title: &str,
    ok_message: &str,
    missing_title: &str,
    missing_message: &str,
    results: &mut Vec<CheckResult>,
) {
    if found {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                ok_title.to_owned(),
                ok_message.to_owned(),
                Some(rel_path.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            missing_title.to_owned(),
            missing_message.to_owned(),
            Some(rel_path.to_owned()),
            None,
            false,
        ));
    }
}

fn line_contains_command(raw: &str, command_text: &str, predicate: fn(&str) -> bool) -> bool {
    let segments = split_command_segments(raw);
    if segments.is_empty() {
        return predicate(command_text);
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

        if reachable && segment.operator_after != Some("&") && segment.operator_after != Some("|") {
            if predicate(&segment.text) {
                return true;
            }
            for substitution in extract_command_substitutions(&segment.text) {
                if line_contains_command(&substitution, &substitution, predicate) {
                    return true;
                }
            }
        }

        if reachable {
            prefix_status = constant_exit_status(&segment.text);
        }
    }

    false
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommandSegment {
    text: String,
    operator_before: Option<&'static str>,
    operator_after: Option<&'static str>,
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
                )),
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

fn is_guardrail_validate_staged_command(command_text: &str) -> bool {
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
        let mut split_string = None;
        while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
            let flag = parts.next().unwrap_or_default();
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
            if env_flag_takes_value(flag) {
                let value = parts.next().unwrap_or_default();
                if matches!(flag, "-S" | "--split-string") {
                    split_string = Some(value.to_owned());
                }
            }
        }
        while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
            let _ = parts.next();
        }

        if let Some(script) = split_string {
            let mut nested = script;
            let tail: Vec<_> = parts.map(str::to_owned).collect();
            if !tail.is_empty() {
                nested.push(' ');
                nested.push_str(&tail.join(" "));
            }
            return line_contains_command(&nested, &nested, is_guardrail_validate_staged_command);
        }

        let Some(next) = parts.next() else {
            return false;
        };
        return is_guardrail_wrapper_or_command(next, &mut parts);
    }

    if matches!(first, "sh" | "bash") {
        return is_shell_wrapper_invocation(&mut parts);
    }
    if first == "command" {
        return is_command_wrapper_invocation(&mut parts);
    }
    if first == "exec" {
        return is_exec_wrapper_invocation(&mut parts);
    }

    first == "guardrail3" && is_guardrail_validate_args(&mut parts)
}

fn is_path_qualified_guardrail_validate_staged_command(command_text: &str) -> bool {
    let tokens = shell_words(command_text);
    let mut parts = tokens.iter().map(String::as_str).peekable();

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    let Some(first) = parts.next() else {
        return false;
    };

    if first == "env" {
        let mut split_string = None;
        while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
            let flag = parts.next().unwrap_or_default();
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
            if env_flag_takes_value(flag) {
                let value = parts.next().unwrap_or_default();
                if matches!(flag, "-S" | "--split-string") {
                    split_string = Some(value.to_owned());
                }
            }
        }
        while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
            let _ = parts.next();
        }

        if let Some(script) = split_string {
            let mut nested = script;
            let tail: Vec<_> = parts.map(str::to_owned).collect();
            if !tail.is_empty() {
                nested.push(' ');
                nested.push_str(&tail.join(" "));
            }
            return line_contains_command(
                &nested,
                &nested,
                is_path_qualified_guardrail_validate_staged_command,
            );
        }

        let Some(next) = parts.next() else {
            return false;
        };
        return is_path_qualified_guardrail_wrapper_or_command(next, &mut parts);
    }

    if !first.contains('/') || normalize_command_token(first) != "guardrail3" {
        return false;
    }

    is_guardrail_validate_args(&mut parts)
}

fn is_guardrail_wrapper_or_command<'a, I>(
    token: &'a str,
    parts: &mut std::iter::Peekable<I>,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    match normalize_command_token(token) {
        "guardrail3" => is_guardrail_validate_args(parts),
        "sh" | "bash" => is_shell_wrapper_invocation(parts),
        "command" => is_command_wrapper_invocation(parts),
        "exec" => is_exec_wrapper_invocation(parts),
        "env" => {
            let mut nested = String::from("env");
            let tail: Vec<_> = parts.map(str::to_owned).collect();
            if !tail.is_empty() {
                nested.push(' ');
                nested.push_str(&tail.join(" "));
            }
            is_guardrail_validate_staged_command(&nested)
        }
        _ => false,
    }
}

fn is_path_qualified_guardrail_wrapper_or_command<'a, I>(
    token: &'a str,
    parts: &mut std::iter::Peekable<I>,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    if token.contains('/') && normalize_command_token(token) == "guardrail3" {
        return is_guardrail_validate_args(parts);
    }

    match normalize_command_token(token) {
        "sh" | "bash" => is_shell_wrapper_invocation_with(
            parts,
            is_path_qualified_guardrail_validate_staged_command,
        ),
        "command" => is_command_wrapper_invocation_with(
            parts,
            is_path_qualified_guardrail_wrapper_or_command,
        ),
        "exec" => {
            is_exec_wrapper_invocation_with(parts, is_path_qualified_guardrail_wrapper_or_command)
        }
        "env" => {
            let mut nested = String::from("env");
            let tail: Vec<_> = parts.map(str::to_owned).collect();
            if !tail.is_empty() {
                nested.push(' ');
                nested.push_str(&tail.join(" "));
            }
            is_path_qualified_guardrail_validate_staged_command(&nested)
        }
        _ => false,
    }
}

fn is_command_wrapper_invocation<'a, I>(parts: &mut std::iter::Peekable<I>) -> bool
where
    I: Iterator<Item = &'a str>,
{
    is_command_wrapper_invocation_with(parts, is_guardrail_wrapper_or_command)
}

fn is_command_wrapper_invocation_with<'a, I>(
    parts: &mut std::iter::Peekable<I>,
    predicate: fn(&'a str, &mut std::iter::Peekable<I>) -> bool,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
        let flag = parts.next().unwrap_or_default();
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
    let Some(next) = parts.next() else {
        return false;
    };
    predicate(next, parts)
}

fn is_exec_wrapper_invocation<'a, I>(parts: &mut std::iter::Peekable<I>) -> bool
where
    I: Iterator<Item = &'a str>,
{
    is_exec_wrapper_invocation_with(parts, is_guardrail_wrapper_or_command)
}

fn is_exec_wrapper_invocation_with<'a, I>(
    parts: &mut std::iter::Peekable<I>,
    predicate: fn(&'a str, &mut std::iter::Peekable<I>) -> bool,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if flag == "--" {
            break;
        }
    }
    let Some(next) = parts.next() else {
        return false;
    };
    predicate(next, parts)
}

fn is_shell_wrapper_invocation<'a, I>(parts: &mut std::iter::Peekable<I>) -> bool
where
    I: Iterator<Item = &'a str>,
{
    is_shell_wrapper_invocation_with(parts, is_guardrail_validate_staged_command)
}

fn is_shell_wrapper_invocation_with<'a, I>(
    parts: &mut std::iter::Peekable<I>,
    predicate: fn(&str) -> bool,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    while let Some(token) = parts.peek().copied() {
        if !token.starts_with('-') {
            break;
        }
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
    }

    let Some(script) = parts.next() else {
        return false;
    };
    line_contains_command(script, script, predicate)
}

fn is_guardrail_validate_args<'a, I>(parts: &mut std::iter::Peekable<I>) -> bool
where
    I: Iterator<Item = &'a str>,
{
    while let Some(token) = parts.peek().copied() {
        if !token.starts_with('-') {
            break;
        }

        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if let Some((flag_name, _)) = flag.split_once('=')
            && guardrail_global_flag_takes_value(flag_name)
        {
            continue;
        }
        if guardrail_global_flag_takes_value(flag) {
            let _ = parts.next();
        }
    }

    let first = match parts.next() {
        Some(value) => value,
        None => return false,
    };

    let saw_validate = if first == "rs" {
        parts.next() == Some("validate")
    } else {
        first == "validate"
    };

    if !saw_validate {
        return false;
    }

    let mut saw_staged = false;
    for token in parts {
        if is_help_or_version_flag(token) {
            return false;
        }
        if token == "--staged" {
            saw_staged = true;
        }
    }

    saw_staged
}

fn guardrail_global_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "--config" | "--format" | "--root" | "--cache-dir")
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
#[path = "tests/steps/hook_rs_08_guardrail_validate_staged_present_tests/mod.rs"]
mod hook_rs_08_guardrail_validate_staged_present_tests;
