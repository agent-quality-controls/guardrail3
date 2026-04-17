use crate::parse_script;
use crate::types::ParsedShellScript;

use super::ResolvedCommand;
use super::lex::{
    constant_exit_status, env_flag_takes_value, env_flag_without_value,
    exec_flag_takes_value, extract_command_substitutions, is_help_or_version_flag,
    is_terminal_exit, looks_like_env_assignment, normalize_command_token, shell_flag_takes_value,
    shell_words, split_command_segments,
};

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

pub(super) fn any_resolved_command<F>(parsed: &ParsedShellScript, predicate: &F) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
    any_resolved_command_with_mode(parsed, predicate, false)
}

pub(super) fn any_resolved_command_on_line<F>(
    parsed: &ParsedShellScript,
    raw: &str,
    line_no: usize,
    predicate: &F,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
    let mut visiting = Vec::new();
    line_matches(raw, parsed, &mut visiting, predicate, line_no)
}

pub(super) fn any_resolved_command_relaxed<F>(parsed: &ParsedShellScript, predicate: &F) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
    any_resolved_command_with_mode(parsed, predicate, true)
}

fn any_resolved_command_with_mode<F>(
    parsed: &ParsedShellScript,
    predicate: &F,
    allow_detached: bool,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
    let mut visiting = Vec::new();
    parsed.executable_lines.iter().any(|line| {
        line_matches_with_mode(
            &line.raw,
            parsed,
            &mut visiting,
            predicate,
            line.line_no,
            allow_detached,
        )
    })
}

fn line_matches<F>(
    raw: &str,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: &F,
    line_no: usize,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
    line_matches_with_mode(raw, root, visiting, predicate, line_no, false)
}

fn line_matches_with_mode<F>(
    raw: &str,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: &F,
    line_no: usize,
    allow_detached: bool,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
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

fn segment_matches<F>(
    segment: &str,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: &F,
    line_no: usize,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
    token_sequence_matches(shell_words(segment), root, visiting, predicate, line_no)
}

fn token_sequence_matches<F>(
    tokens: Vec<String>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: &F,
    line_no: usize,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
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

fn dispatch_token<F>(
    token: &str,
    cursor: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: &F,
    line_no: usize,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
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

fn dispatch_external_token<F>(
    token: &str,
    cursor: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: &F,
    line_no: usize,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
    match normalize_command_token(token) {
        "env" => env_wrapper_matches(cursor, root, visiting, predicate, line_no),
        "sh" | "bash" => shell_wrapper_matches(cursor, root, visiting, predicate, line_no),
        "command" => command_wrapper_matches(cursor, root, visiting, predicate, line_no),
        "exec" => exec_wrapper_matches(cursor, root, visiting, predicate, line_no),
        _ => predicate(&resolved_command(token, cursor, line_no)),
    }
}

fn function_defined_before(command_name: &str, root: &ParsedShellScript, line_no: usize) -> bool {
    root.functions
        .iter()
        .any(|function| function.name == command_name && function.line_no <= line_no)
}

fn called_function_matches<F>(
    command_name: &str,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: &F,
    line_no: usize,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
    let Some(function) = root
        .functions
        .iter()
        .find(|function| function.name == command_name && function.line_no <= line_no)
    else {
        return false;
    };
    if visiting.iter().any(|name| name == &function.name) {
        return false;
    }

    visiting.push(function.name.clone());
    let body = parse_script(&function.body);
    let found = body.executable_lines.iter().any(|line| {
        line_matches(&line.raw, &body, visiting, predicate, line.line_no)
            || line_matches(&line.raw, root, visiting, predicate, line.line_no)
    });
    let _ = visiting.pop();
    found
}

fn resolved_command(token: &str, cursor: &TokenCursor<'_>, line_no: usize) -> ResolvedCommand {
    let mut tokens = vec![token.to_owned()];
    tokens.extend(cursor.remaining().iter().cloned());
    let command_text = tokens.join(" ");

    ResolvedCommand::new(
        line_no,
        command_text,
        token.to_owned(),
        normalize_command_token(token).to_owned(),
        tokens,
    )
}

fn env_wrapper_matches<F>(
    cursor: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: &F,
    line_no: usize,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
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

fn shell_wrapper_matches<F>(
    cursor: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: &F,
    line_no: usize,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
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

fn command_wrapper_matches<F>(
    cursor: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: &F,
    line_no: usize,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
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

fn exec_wrapper_matches<F>(
    cursor: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    predicate: &F,
    line_no: usize,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
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
