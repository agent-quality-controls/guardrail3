#![allow(
    clippy::missing_docs_in_private_items,
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    clippy::string_slice,
    clippy::excessive_nesting,
    reason = "lex.rs IS the bash-style tokenizer; byte offsets, slicing, and arithmetic mirror the lexer's character-stream invariants"
)]

use super::api::CommandSegment;

pub(super) fn split_command_segments(raw: &str) -> Vec<CommandSegment> {
    crate::shell_ast::command_segments(raw)
        .into_iter()
        .map(|segment| CommandSegment {
            text: segment.text,
            operator_before: segment.operator_before,
            operator_after: segment.operator_after,
        })
        .collect()
}

pub(super) fn constant_exit_status(segment: &str) -> Option<bool> {
    crate::shell_ast::constant_exit_status(segment)
}

pub(super) fn is_terminal_exit(segment: &str) -> bool {
    crate::shell_ast::is_terminal_exit(segment)
}

pub(super) fn shell_words(command_text: &str) -> Vec<String> {
    crate::shell_ast::shell_words(command_text)
}

pub(super) fn extract_command_substitutions(line: &str) -> Vec<String> {
    crate::shell_ast::command_substitutions(line)
}

pub(super) fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

pub(super) fn env_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "-u" | "--unset" | "-C" | "--chdir" | "-S" | "--split-string"
    )
}

pub(super) fn env_flag_without_value(flag: &str) -> bool {
    matches!(flag, "-i" | "--ignore-environment")
}

pub(super) fn shell_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "-c" | "-o" | "-O" | "--init-file" | "--rcfile")
}

pub(super) fn exec_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "-a")
}

pub(super) fn normalize_command_token(token: &str) -> &str {
    token.rsplit('/').next().unwrap_or(token)
}

pub(super) fn looks_like_env_assignment(token: &str) -> bool {
    let Some((name, _)) = token.split_once('=') else {
        return false;
    };
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}
