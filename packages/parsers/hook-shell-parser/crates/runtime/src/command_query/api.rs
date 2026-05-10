#![allow(
    clippy::missing_docs_in_private_items,
    clippy::too_many_arguments,
    reason = "command_query::api is the public DSL surface for hook rules; each helper exposes the exact knobs callers need to ask a structured question about a script"
)]

use super::engine;

use crate::types::ParsedShellScript;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CommandSegment {
    pub(super) text: String,
    pub(super) operator_before: Option<&'static str>,
    pub(super) operator_after: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedCommand {
    line_no: usize,
    command_text: String,
    command_path: String,
    command_name: String,
    tokens: Vec<String>,
}

impl ResolvedCommand {
    pub(super) const fn new(
        line_no: usize,
        command_text: String,
        command_path: String,
        command_name: String,
        tokens: Vec<String>,
    ) -> Self {
        Self {
            line_no,
            command_text,
            command_path,
            command_name,
            tokens,
        }
    }

    #[must_use]
    pub const fn line_no(&self) -> usize {
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CommandQueryOptions {
    allow_detached: bool,
    allow_forward_functions: bool,
}

impl CommandQueryOptions {
    #[must_use]
    pub const fn with_detached_commands(mut self) -> Self {
        self.allow_detached = true;
        self
    }

    #[must_use]
    pub const fn with_forward_functions(mut self) -> Self {
        self.allow_forward_functions = true;
        self
    }

    #[must_use]
    pub(super) const fn allow_detached(self) -> bool {
        self.allow_detached
    }

    #[must_use]
    pub(super) const fn allow_forward_functions(self) -> bool {
        self.allow_forward_functions
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandVisit {
    Continue,
    Stop,
}

pub trait ShellEnvState: Clone {
    fn apply_assignment(&mut self, name: &str, value: &str);

    fn unset(&mut self, name: &str);

    fn clear(&mut self);
}

#[must_use]
pub fn any_resolved_command(
    parsed: &ParsedShellScript,
    predicate: impl Fn(&ResolvedCommand) -> bool,
) -> bool {
    engine::any_resolved_command(parsed, &predicate)
}

#[must_use]
pub fn any_resolved_command_on_line(
    parsed: &ParsedShellScript,
    raw: &str,
    line_no: usize,
    predicate: impl Fn(&ResolvedCommand) -> bool,
) -> bool {
    engine::any_resolved_command_on_line(parsed, raw, line_no, &predicate)
}

#[must_use]
pub fn any_resolved_command_on_line_in_context(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    raw: &str,
    line_no: usize,
    root_line_no: usize,
    predicate: impl Fn(&ResolvedCommand) -> bool,
) -> bool {
    engine::any_resolved_command_on_line_in_context(
        local,
        root,
        raw,
        line_no,
        root_line_no,
        &predicate,
    )
}

#[must_use]
pub fn any_resolved_command_relaxed(
    parsed: &ParsedShellScript,
    predicate: impl Fn(&ResolvedCommand) -> bool,
) -> bool {
    engine::any_resolved_command_relaxed(parsed, &predicate)
}

pub fn visit_resolved_commands_with_env<S>(
    parsed: &ParsedShellScript,
    initial_state: S,
    options: CommandQueryOptions,
    visitor: impl FnMut(&ResolvedCommand, &S) -> CommandVisit,
) where
    S: ShellEnvState,
{
    engine::visit_resolved_commands_with_env(parsed, initial_state, options, visitor);
}

#[must_use]
pub fn shell_words(command_text: &str) -> Vec<String> {
    crate::shell_ast::shell_words(command_text)
}

#[cfg(test)]
pub(super) use crate::parse_script as parse_script_for_tests;

#[cfg(test)]
#[path = "api_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod api_tests;
