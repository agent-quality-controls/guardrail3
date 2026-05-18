#![allow(
    clippy::missing_docs_in_private_items,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::indexing_slicing,
    clippy::string_slice,
    clippy::type_complexity,
    reason = "wrappers.rs unwraps real shell wrappers (xargs, env, sudo, ...) around tools; each detector exposes the exact wrapper-specific knobs callers need and the per-wrapper helper signatures intentionally surface the wrapper's CLI surface"
)]

use crate::types::ParsedShellScript;

use super::{CommandQueryOptions, CommandVisit, ResolvedCommand, ShellEnvState};
use super::{engine, lex, state};

pub(super) fn env_wrapper_visits<S, F>(
    cursor: &mut state::TokenCursor<'_>,
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    state: &mut S,
    visitor: &mut F,
    line_no: usize,
    root_line_no: usize,
    options: CommandQueryOptions,
) -> bool
where
    S: ShellEnvState,
    F: FnMut(&ResolvedCommand, &S) -> CommandVisit,
{
    let mut split_string = None;

    while cursor.peek().is_some_and(|token| token.starts_with('-')) {
        let flag = cursor.next().unwrap_or_default();
        if lex::is_help_or_version_flag(flag) {
            return false;
        }
        if flag == "--" {
            break;
        }
        match flag.split_once('=') {
            Some((flag_name, value)) if lex::env_flag_takes_value(flag_name) => {
                match flag_name {
                    "-u" | "--unset" => state.unset(value),
                    "-S" | "--split-string" => split_string = Some(value.to_owned()),
                    _ => {}
                }
                continue;
            }
            _ => {}
        }
        if lex::env_flag_without_value(flag) {
            if matches!(flag, "-i" | "--ignore-environment") {
                state.clear();
            }
            continue;
        }
        if lex::env_flag_takes_value(flag) {
            let value = cursor.next().unwrap_or_default();
            match flag {
                "-u" | "--unset" => state.unset(value),
                "-S" | "--split-string" => split_string = Some(value.to_owned()),
                _ => {}
            }
            continue;
        }

        return false;
    }

    while cursor.peek().is_some_and(lex::looks_like_env_assignment) {
        let token = cursor.next().unwrap_or_default();
        state::apply_assignment_token(token, state);
    }

    if let Some(script) = split_string {
        if cursor.remaining().is_empty() {
            // Keep the split string as the command below.
        } else if let Some((name, value)) = assignment_parts(&script) {
            state.apply_assignment(name, value);
            let Some(next) = cursor.next() else {
                return false;
            };
            return engine::dispatch_external_token(
                next,
                cursor,
                local,
                root,
                visiting,
                state,
                visitor,
                line_no,
                root_line_no,
                options,
            );
        }

        let mut split_tokens =
            if !cursor.remaining().is_empty() && lex::looks_like_env_assignment(&script) {
                vec![script]
            } else {
                lex::shell_words(&script)
            };
        split_tokens.extend(cursor.remaining().iter().cloned());
        return engine::segment_visits(
            split_tokens,
            local,
            root,
            visiting,
            state,
            visitor,
            line_no,
            root_line_no,
            options,
        )
        .stopped;
    }

    let Some(next) = cursor.next() else {
        return false;
    };

    engine::dispatch_external_token(
        next,
        cursor,
        local,
        root,
        visiting,
        state,
        visitor,
        line_no,
        root_line_no,
        options,
    )
}

pub(super) fn shell_wrapper_visits<S, F>(
    cursor: &mut state::TokenCursor<'_>,
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    state: &mut S,
    visitor: &mut F,
    line_no: usize,
    root_line_no: usize,
    options: CommandQueryOptions,
) -> bool
where
    S: ShellEnvState,
    F: FnMut(&ResolvedCommand, &S) -> CommandVisit,
{
    let mut script = None;

    while let Some(token) = cursor.peek() {
        if !token.starts_with('-') {
            break;
        }

        let flag = cursor.next().unwrap_or_default();
        if lex::is_help_or_version_flag(flag) {
            return false;
        }

        match flag.split_once('=') {
            Some((flag_name, value)) if lex::shell_flag_takes_value(flag_name) => {
                if flag_name == "-c" {
                    script = Some(value.to_owned());
                }
                continue;
            }
            _ => {}
        }

        if shell_cluster_uses_next_script(flag) {
            script = Some(cursor.next().unwrap_or_default().to_owned());
            continue;
        }

        if lex::shell_flag_takes_value(flag) {
            let value = cursor.next().unwrap_or_default();
            if flag == "-c" {
                script = Some(value.to_owned());
            }
            continue;
        }

        return false;
    }

    if let Some(script) = script {
        let script = unquote_shell_word(&script);
        return engine::line_visits_with_mode(
            script,
            local,
            root,
            visiting,
            state,
            visitor,
            line_no,
            root_line_no,
            options,
        );
    }

    let Some(next) = cursor.next() else {
        return false;
    };

    engine::dispatch_external_token(
        next,
        cursor,
        local,
        root,
        visiting,
        state,
        visitor,
        line_no,
        root_line_no,
        options,
    )
}

pub(super) fn command_wrapper_visits<S, F>(
    cursor: &mut state::TokenCursor<'_>,
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    state: &mut S,
    visitor: &mut F,
    line_no: usize,
    root_line_no: usize,
    options: CommandQueryOptions,
) -> bool
where
    S: ShellEnvState,
    F: FnMut(&ResolvedCommand, &S) -> CommandVisit,
{
    while cursor.peek().is_some_and(|token| token.starts_with('-')) {
        let flag = cursor.next().unwrap_or_default();
        if lex::is_help_or_version_flag(flag) || matches!(flag, "-v" | "-V") {
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

    engine::dispatch_external_token(
        next,
        cursor,
        local,
        root,
        visiting,
        state,
        visitor,
        line_no,
        root_line_no,
        options,
    )
}

pub(super) fn exec_wrapper_visits<S, F>(
    cursor: &mut state::TokenCursor<'_>,
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    state: &mut S,
    visitor: &mut F,
    line_no: usize,
    root_line_no: usize,
    options: CommandQueryOptions,
) -> bool
where
    S: ShellEnvState,
    F: FnMut(&ResolvedCommand, &S) -> CommandVisit,
{
    while cursor.peek().is_some_and(|token| token.starts_with('-')) {
        let flag = cursor.next().unwrap_or_default();
        if lex::is_help_or_version_flag(flag) {
            return false;
        }
        if flag == "--" {
            break;
        }
        if lex::exec_flag_takes_value(flag) {
            let _ = cursor.next();
            continue;
        }

        return false;
    }

    let Some(next) = cursor.next() else {
        return false;
    };

    engine::dispatch_external_token(
        next,
        cursor,
        local,
        root,
        visiting,
        state,
        visitor,
        line_no,
        root_line_no,
        options,
    )
}

fn shell_cluster_uses_next_script(flag: &str) -> bool {
    if !flag.starts_with('-') || flag.starts_with("--") {
        return false;
    }

    let short = &flag[1..];
    short.contains('c')
}

fn assignment_parts(token: &str) -> Option<(&str, &str)> {
    let token = unquote_shell_word(token);
    let (name, value) = token.split_once('=')?;
    let mut chars = name.chars();
    let first = chars.next()?;
    if !(first.is_ascii_alphabetic() || first == '_') {
        return None;
    }
    chars
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        .then_some((name, value))
}

fn unquote_shell_word(token: &str) -> &str {
    token
        .strip_prefix('\'')
        .and_then(|value| value.strip_suffix('\''))
        .or_else(|| {
            token
                .strip_prefix('"')
                .and_then(|value| value.strip_suffix('"'))
        })
        .unwrap_or(token)
}
