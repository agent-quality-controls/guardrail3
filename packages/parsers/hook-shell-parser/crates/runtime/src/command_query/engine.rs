use crate::types::ParsedShellScript;

use super::{CommandQueryOptions, CommandVisit, ResolvedCommand, ShellEnvState};
use super::{lex, state, wrappers};

pub(super) fn any_resolved_command<F>(parsed: &ParsedShellScript, predicate: &F) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
    let mut found = false;
    visit_resolved_commands_with_env(
        parsed,
        state::NoEnvState,
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
    any_resolved_command_on_line_in_context(parsed, parsed, raw, line_no, line_no, predicate)
}

pub(super) fn any_resolved_command_on_line_in_context<F>(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    raw: &str,
    line_no: usize,
    root_line_no: usize,
    predicate: &F,
) -> bool
where
    F: Fn(&ResolvedCommand) -> bool,
{
    let mut visiting = Vec::new();
    let mut state = state::NoEnvState;
    let mut found = false;
    let _ = line_visits_with_mode(
        raw,
        local,
        root,
        &mut visiting,
        &mut state,
        &mut |command: &ResolvedCommand, _state: &state::NoEnvState| {
            if predicate(command) {
                found = true;
                CommandVisit::Stop
            } else {
                CommandVisit::Continue
            }
        },
        line_no,
        root_line_no,
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
        state::NoEnvState,
        CommandQueryOptions::default().with_detached_commands(),
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
            line.line_no,
            options,
        ) {
            break;
        }
    }
}

pub(super) fn line_visits_with_mode<S, F>(
    raw: &str,
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
    let segments = lex::split_command_segments(raw);
    if segments.is_empty() {
        return segment_visits(
            lex::shell_words(raw),
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
        if reachable && (options.allow_detached() || !detached) {
            let mut segment_state = state.clone();
            let outcome = segment_visits(
                lex::shell_words(&segment.text),
                local,
                root,
                visiting,
                &mut segment_state,
                visitor,
                line_no,
                root_line_no,
                options,
            );
            if outcome.stopped {
                return true;
            }
            if outcome.persist_state {
                *state = segment_state;
            }

            for substitution in lex::extract_command_substitutions(&segment.text) {
                let mut substitution_state = state.clone();
                if line_visits_with_mode(
                    &substitution,
                    local,
                    root,
                    visiting,
                    &mut substitution_state,
                    visitor,
                    line_no,
                    root_line_no,
                    options,
                ) {
                    return true;
                }
            }
        }

        if reachable {
            if lex::is_terminal_exit(&segment.text) {
                break;
            }
            prefix_status = lex::constant_exit_status(&segment.text);
        }
    }

    false
}

pub(super) fn segment_visits<S, F>(
    tokens: Vec<String>,
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    visiting: &mut Vec<String>,
    state: &mut S,
    visitor: &mut F,
    line_no: usize,
    root_line_no: usize,
    options: CommandQueryOptions,
) -> state::SegmentOutcome
where
    S: ShellEnvState,
    F: FnMut(&ResolvedCommand, &S) -> CommandVisit,
{
    let mut cursor = state::TokenCursor::new(&tokens);

    while cursor.peek() == Some("!") {
        let _ = cursor.next();
    }

    let mut local_state = state.clone();
    let mut has_local_overlay = false;
    while cursor.peek().is_some_and(lex::looks_like_env_assignment) {
        let token = cursor.next().unwrap_or_default();
        state::apply_assignment_token(token, &mut local_state);
        has_local_overlay = true;
    }

    let Some(first) = cursor.next() else {
        return state::SegmentOutcome {
            stopped: false,
            persist_state: false,
        };
    };

    match lex::normalize_command_token(first) {
        "export" => {
            state::apply_export_assignments(&mut cursor, state);
            state::SegmentOutcome {
                stopped: false,
                persist_state: true,
            }
        }
        "unset" => {
            state::apply_unset_arguments(&mut cursor, state);
            state::SegmentOutcome {
                stopped: false,
                persist_state: true,
            }
        }
        command_name if function_defined(command_name, local, line_no, options) => {
            if has_local_overlay {
                let mut function_state = local_state;
                state::SegmentOutcome {
                    stopped: called_function_visits(
                        command_name,
                        local,
                        root,
                        visiting,
                        &mut function_state,
                        visitor,
                        line_no,
                        root_line_no,
                        options,
                    ),
                    persist_state: false,
                }
            } else {
                state::SegmentOutcome {
                    stopped: called_function_visits(
                        command_name,
                        local,
                        root,
                        visiting,
                        state,
                        visitor,
                        line_no,
                        root_line_no,
                        options,
                    ),
                    persist_state: true,
                }
            }
        }
        command_name
            if !std::ptr::eq(local, root)
                && function_defined(command_name, root, root_line_no, options) =>
        {
            if has_local_overlay {
                let mut function_state = local_state;
                state::SegmentOutcome {
                    stopped: called_function_visits(
                        command_name,
                        root,
                        root,
                        visiting,
                        &mut function_state,
                        visitor,
                        root_line_no,
                        root_line_no,
                        options,
                    ),
                    persist_state: false,
                }
            } else {
                state::SegmentOutcome {
                    stopped: called_function_visits(
                        command_name,
                        root,
                        root,
                        visiting,
                        state,
                        visitor,
                        root_line_no,
                        root_line_no,
                        options,
                    ),
                    persist_state: true,
                }
            }
        }
        _ => state::SegmentOutcome {
            stopped: dispatch_external_token(
                first,
                &mut cursor,
                local,
                root,
                visiting,
                &mut local_state,
                visitor,
                line_no,
                root_line_no,
                options,
            ),
            persist_state: false,
        },
    }
}

pub(super) fn dispatch_external_token<S, F>(
    token: &str,
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
    match lex::normalize_command_token(token) {
        "env" => wrappers::env_wrapper_visits(
            cursor, local, root, visiting, state, visitor, line_no, root_line_no, options,
        ),
        "sh" | "bash" => wrappers::shell_wrapper_visits(
            cursor, local, root, visiting, state, visitor, line_no, root_line_no, options,
        ),
        "command" => wrappers::command_wrapper_visits(
            cursor, local, root, visiting, state, visitor, line_no, root_line_no, options,
        ),
        "exec" => wrappers::exec_wrapper_visits(
            cursor, local, root, visiting, state, visitor, line_no, root_line_no, options,
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
    matching_function(command_name, parsed, line_no, options).is_some()
}

fn called_function_visits<S, F>(
    command_name: &str,
    lookup: &ParsedShellScript,
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
    let Some(function) = matching_function(command_name, lookup, line_no, options) else {
        return false;
    };
    if visiting.iter().any(|name| name == &function.name) {
        return false;
    }

    visiting.push(function.name.clone());
    let mut stopped = false;
    for line in &function.parsed_body.executable_lines {
        if line_visits_with_mode(
            &line.raw,
            &function.parsed_body,
            root,
            visiting,
            state,
            visitor,
            line.line_no,
            root_line_no,
            options,
        ) {
            stopped = true;
            break;
        }
    }
    let _ = visiting.pop();
    stopped
}

fn resolved_command(
    token: &str,
    cursor: &state::TokenCursor<'_>,
    line_no: usize,
) -> ResolvedCommand {
    let mut tokens = vec![token.to_owned()];
    tokens.extend(cursor.remaining().iter().cloned());
    let command_text = tokens.join(" ");

    ResolvedCommand::new(
        line_no,
        command_text,
        token.to_owned(),
        lex::normalize_command_token(token).to_owned(),
        tokens,
    )
}

fn matching_function<'a>(
    command_name: &str,
    parsed: &'a ParsedShellScript,
    line_no: usize,
    options: CommandQueryOptions,
) -> Option<&'a crate::types::ShellFunction> {
    parsed.functions.iter().rev().find(|function| {
        function.name == command_name
            && (options.allow_forward_functions() || function.line_no <= line_no)
    })
}
