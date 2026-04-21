use crate::parse_script;
use crate::types::ParsedShellScript;

use super::lex::{
    constant_exit_status, env_flag_takes_value, env_flag_without_value, exec_flag_takes_value,
    extract_command_substitutions, is_help_or_version_flag, is_terminal_exit,
    looks_like_env_assignment, normalize_command_token, shell_flag_takes_value, shell_words,
    split_command_segments,
};
use super::{CommandQueryOptions, CommandVisit, ResolvedCommand, ShellEnvState};

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

#[derive(Debug, Clone, Default)]
struct NoEnvState;

impl ShellEnvState for NoEnvState {
    fn apply_assignment(&mut self, _name: &str, _value: &str) {}

    fn unset(&mut self, _name: &str) {}

    fn clear(&mut self) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SegmentOutcome {
    stopped: bool,
    persist_state: bool,
}

pub(super) fn any_resolved_command<F>(parsed: &ParsedShellScript, predicate: &F) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
    let mut found = false;
    visit_resolved_commands_with_env(
        parsed,
        NoEnvState,
        CommandQueryOptions::default(),
        |command, _state| {
            if predicate(command) {
                found = true;
                CommandVisit::Stop
            } else {
                CommandVisit::Continue
            }
        },
    );
    found
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
    let mut state = NoEnvState;
    let mut found = false;
    let _ = line_visits_with_mode(
        raw,
        parsed,
        parsed,
        &mut visiting,
        &mut state,
        &mut |command: &ResolvedCommand, _state: &NoEnvState| {
            if predicate(command) {
                found = true;
                CommandVisit::Stop
            } else {
                CommandVisit::Continue
            }
        },
        line_no,
        CommandQueryOptions::default(),
    );
    found
}

pub(super) fn any_resolved_command_relaxed<F>(parsed: &ParsedShellScript, predicate: &F) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
    let mut found = false;
    visit_resolved_commands_with_env(
        parsed,
        NoEnvState,
        CommandQueryOptions {
            allow_detached: true,
            allow_forward_functions: false,
        },
        |command, _state| {
            if predicate(command) {
                found = true;
                CommandVisit::Stop
            } else {
                CommandVisit::Continue
            }
        },
    );
    found
}

pub(super) fn visit_resolved_commands_with_env<S, F>(
    parsed: &ParsedShellScript,
    initial_state: S,
    options: CommandQueryOptions,
    mut visitor: F,
) where
    S: ShellEnvState,
    F: FnMut(&ResolvedCommand, &S) -> CommandVisit,
{
    let mut visiting = Vec::new();
    let mut state = initial_state;
    for line in &parsed.executable_lines {
        if line_visits_with_mode(
            &line.raw,
            parsed,
            parsed,
            &mut visiting,
            &mut state,
            &mut visitor,
            line.line_no,
            options,
        ) {
            break;
        }
    }
}

fn line_visits_with_mode<S, F>(
    raw: &str,
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    state: &mut S,
    visitor: &mut F,
    line_no: usize,
    options: CommandQueryOptions,
) -> bool
where
    S: ShellEnvState,
    F: FnMut(&ResolvedCommand, &S) -> CommandVisit,
{
    let segments = split_command_segments(raw);
    if segments.is_empty() {
        return segment_visits(
            shell_words(raw),
            local,
            root,
            visiting,
            state,
            visitor,
            line_no,
            options,
        )
        .stopped;
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
        if reachable && (options.allow_detached || !detached) {
            let mut segment_state = state.clone();
            let outcome = segment_visits(
                shell_words(&segment.text),
                local,
                root,
                visiting,
                &mut segment_state,
                visitor,
                line_no,
                options,
            );
            if outcome.stopped {
                return true;
            }
            if outcome.persist_state {
                *state = segment_state;
            }

            for substitution in extract_command_substitutions(&segment.text) {
                let mut substitution_state = state.clone();
                if line_visits_with_mode(
                    &substitution,
                    local,
                    root,
                    visiting,
                    &mut substitution_state,
                    visitor,
                    line_no,
                    options,
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

fn segment_visits<S, F>(
    tokens: Vec<String>,
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    state: &mut S,
    visitor: &mut F,
    line_no: usize,
    options: CommandQueryOptions,
) -> SegmentOutcome
where
    S: ShellEnvState,
    F: FnMut(&ResolvedCommand, &S) -> CommandVisit,
{
    let mut cursor = TokenCursor::new(&tokens);

    while cursor.peek() == Some("!") {
        let _ = cursor.next();
    }

    let mut local_state = state.clone();
    let mut has_local_overlay = false;
    while cursor.peek().is_some_and(looks_like_env_assignment) {
        let token = cursor.next().unwrap_or_default();
        apply_assignment_token(token, &mut local_state);
        has_local_overlay = true;
    }

    let Some(first) = cursor.next() else {
        return SegmentOutcome {
            stopped: false,
            persist_state: false,
        };
    };

    match normalize_command_token(first) {
        "export" => {
            apply_export_assignments(&mut cursor, state);
            SegmentOutcome {
                stopped: false,
                persist_state: true,
            }
        }
        "unset" => {
            apply_unset_arguments(&mut cursor, state);
            SegmentOutcome {
                stopped: false,
                persist_state: true,
            }
        }
        command_name if function_defined(command_name, local, line_no, options) => {
            if has_local_overlay {
                let mut function_state = local_state;
                SegmentOutcome {
                    stopped: called_function_visits(
                        command_name,
                        local,
                        root,
                        visiting,
                        &mut function_state,
                        visitor,
                        line_no,
                        options,
                    ),
                    persist_state: false,
                }
            } else {
                SegmentOutcome {
                    stopped: called_function_visits(
                        command_name,
                        local,
                        root,
                        visiting,
                        state,
                        visitor,
                        line_no,
                        options,
                    ),
                    persist_state: true,
                }
            }
        }
        command_name
            if !std::ptr::eq(local, root)
                && function_defined(command_name, root, line_no, options) =>
        {
            if has_local_overlay {
                let mut function_state = local_state;
                SegmentOutcome {
                    stopped: called_function_visits(
                        command_name,
                        root,
                        root,
                        visiting,
                        &mut function_state,
                        visitor,
                        line_no,
                        options,
                    ),
                    persist_state: false,
                }
            } else {
                SegmentOutcome {
                    stopped: called_function_visits(
                        command_name,
                        root,
                        root,
                        visiting,
                        state,
                        visitor,
                        line_no,
                        options,
                    ),
                    persist_state: true,
                }
            }
        }
        _ => SegmentOutcome {
            stopped: dispatch_external_token(
                first,
                &mut cursor,
                local,
                root,
                visiting,
                &mut local_state,
                visitor,
                line_no,
                options,
            ),
            persist_state: false,
        },
    }
}

fn dispatch_external_token<S, F>(
    token: &str,
    cursor: &mut TokenCursor<'_>,
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    state: &mut S,
    visitor: &mut F,
    line_no: usize,
    options: CommandQueryOptions,
) -> bool
where
    S: ShellEnvState,
    F: FnMut(&ResolvedCommand, &S) -> CommandVisit,
{
    match normalize_command_token(token) {
        "env" => env_wrapper_visits(
            cursor, local, root, visiting, state, visitor, line_no, options,
        ),
        "sh" | "bash" => shell_wrapper_visits(
            cursor, local, root, visiting, state, visitor, line_no, options,
        ),
        "command" => command_wrapper_visits(
            cursor, local, root, visiting, state, visitor, line_no, options,
        ),
        "exec" => exec_wrapper_visits(
            cursor, local, root, visiting, state, visitor, line_no, options,
        ),
        _ => matches!(
            visitor(&resolved_command(token, cursor, line_no), state),
            CommandVisit::Stop
        ),
    }
}

fn function_defined(
    command_name: &str,
    parsed: &ParsedShellScript,
    line_no: usize,
    options: CommandQueryOptions,
) -> bool {
    parsed.functions.iter().any(|function| {
        function.name == command_name
            && (options.allow_forward_functions || function.line_no <= line_no)
    })
}

fn called_function_visits<S, F>(
    command_name: &str,
    lookup: &ParsedShellScript,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    state: &mut S,
    visitor: &mut F,
    line_no: usize,
    options: CommandQueryOptions,
) -> bool
where
    S: ShellEnvState,
    F: FnMut(&ResolvedCommand, &S) -> CommandVisit,
{
    let Some(function) = lookup.functions.iter().find(|function| {
        function.name == command_name
            && (options.allow_forward_functions || function.line_no <= line_no)
    }) else {
        return false;
    };
    if visiting.iter().any(|name| name == &function.name) {
        return false;
    }

    visiting.push(function.name.clone());
    let body = parse_script(&function.body);
    let mut stopped = false;
    for line in &body.executable_lines {
        if line_visits_with_mode(
            &line.raw,
            &body,
            root,
            visiting,
            state,
            visitor,
            line.line_no,
            options,
        ) {
            stopped = true;
            break;
        }
    }
    let _ = visiting.pop();
    stopped
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

fn env_wrapper_visits<S, F>(
    cursor: &mut TokenCursor<'_>,
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    state: &mut S,
    visitor: &mut F,
    line_no: usize,
    options: CommandQueryOptions,
) -> bool
where
    S: ShellEnvState,
    F: FnMut(&ResolvedCommand, &S) -> CommandVisit,
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
            match flag_name {
                "-u" | "--unset" => state.unset(value),
                "-S" | "--split-string" => split_string = Some(value.to_owned()),
                _ => {}
            }
            continue;
        }
        if env_flag_without_value(flag) {
            if matches!(flag, "-i" | "--ignore-environment") {
                state.clear();
            }
            continue;
        }
        if env_flag_takes_value(flag) {
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

    while cursor.peek().is_some_and(looks_like_env_assignment) {
        let token = cursor.next().unwrap_or_default();
        apply_assignment_token(token, state);
    }

    if let Some(script) = split_string {
        let mut split_tokens =
            if !cursor.remaining().is_empty() && looks_like_env_assignment(&script) {
                vec![script]
            } else {
                shell_words(&script)
            };
        split_tokens.extend(cursor.remaining().iter().cloned());
        return segment_visits(
            split_tokens,
            local,
            root,
            visiting,
            state,
            visitor,
            line_no,
            options,
        )
        .stopped;
    }

    let Some(next) = cursor.next() else {
        return false;
    };

    dispatch_external_token(
        next, cursor, local, root, visiting, state, visitor, line_no, options,
    )
}

fn shell_wrapper_visits<S, F>(
    cursor: &mut TokenCursor<'_>,
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    state: &mut S,
    visitor: &mut F,
    line_no: usize,
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
        return line_visits_with_mode(
            &script, local, root, visiting, state, visitor, line_no, options,
        );
    }

    let Some(next) = cursor.next() else {
        return false;
    };

    dispatch_external_token(
        next, cursor, local, root, visiting, state, visitor, line_no, options,
    )
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

fn command_wrapper_visits<S, F>(
    cursor: &mut TokenCursor<'_>,
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    state: &mut S,
    visitor: &mut F,
    line_no: usize,
    options: CommandQueryOptions,
) -> bool
where
    S: ShellEnvState,
    F: FnMut(&ResolvedCommand, &S) -> CommandVisit,
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

    dispatch_external_token(
        next, cursor, local, root, visiting, state, visitor, line_no, options,
    )
}

fn exec_wrapper_visits<S, F>(
    cursor: &mut TokenCursor<'_>,
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    state: &mut S,
    visitor: &mut F,
    line_no: usize,
    options: CommandQueryOptions,
) -> bool
where
    S: ShellEnvState,
    F: FnMut(&ResolvedCommand, &S) -> CommandVisit,
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

    dispatch_external_token(
        next, cursor, local, root, visiting, state, visitor, line_no, options,
    )
}

fn apply_assignment_token<S: ShellEnvState>(token: &str, state: &mut S) {
    let Some((name, value)) = token.split_once('=') else {
        return;
    };
    state.apply_assignment(name, value);
}

fn apply_export_assignments<S: ShellEnvState>(cursor: &mut TokenCursor<'_>, state: &mut S) {
    while let Some(token) = cursor.next() {
        apply_assignment_token(token, state);
    }
}

fn apply_unset_arguments<S: ShellEnvState>(cursor: &mut TokenCursor<'_>, state: &mut S) {
    while let Some(token) = cursor.next() {
        if token.starts_with('-') {
            continue;
        }
        state.unset(token);
    }
}
