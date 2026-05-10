//! Public command-query DSL plus its lexer, engine, and shell-environment state.

#![allow(
    clippy::module_name_repetitions,
    reason = "command_query module re-exports CommandQueryOptions etc. whose names intentionally encode the DSL's domain"
)]

/// Public API surface for the command-query DSL.
mod api;
/// Visit-driving engine that walks a script and yields `ResolvedCommand`s.
mod engine;
/// Bash-style tokenizer that produces the command-query word stream.
mod lex;
/// Shell environment state machine that tracks variable assignments and exports.
mod state;
/// Detects wrapper invocations (`xargs`, `env`, `sudo`, etc.) around real tools.
mod wrappers;

pub use api::{
    CommandQueryOptions, CommandVisit, ResolvedCommand, ShellEnvState, any_resolved_command,
    any_resolved_command_on_line, any_resolved_command_on_line_in_context,
    any_resolved_command_relaxed, shell_words, visit_resolved_commands_with_env,
};
