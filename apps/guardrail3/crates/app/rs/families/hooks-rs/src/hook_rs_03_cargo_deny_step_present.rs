use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-03";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = input
        .parsed
        .executable_lines
        .iter()
        .any(|line| is_cargo_deny_check_command(line.command_text));

    if found {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "cargo-deny step present".to_owned(),
                message: "Hook runs cargo-deny.".to_owned(),
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
            title: "cargo-deny step missing".to_owned(),
            message: "Hook does not execute cargo-deny.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

fn is_cargo_deny_check_command(command_text: &str) -> bool {
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
        while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
            let _ = parts.next();
        }
        while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
            let _ = parts.next();
        }
        let Some(next) = parts.next() else {
            return false;
        };
        return match normalize_command_token(next) {
            "cargo" => is_cargo_deny_check_invocation(&mut parts),
            "cargo-deny" => is_cargo_deny_binary_check_invocation(&mut parts),
            _ => false,
        };
    }

    match first {
        "cargo" => is_cargo_deny_check_invocation(&mut parts),
        "cargo-deny" => is_cargo_deny_binary_check_invocation(&mut parts),
        _ => false,
    }
}

fn is_cargo_deny_check_invocation<'a, I>(parts: &mut std::iter::Peekable<I>) -> bool
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

    if parts.next() != Some("deny") || parts.next() != Some("check") {
        return false;
    }

    !parts.any(is_help_or_version_flag)
}

fn is_cargo_deny_binary_check_invocation<'a, I>(parts: &mut I) -> bool
where
    I: Iterator<Item = &'a str>,
{
    if parts.next() != Some("check") {
        return false;
    }

    !parts.any(is_help_or_version_flag)
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
#[path = "hook_rs_03_cargo_deny_step_present_tests/mod.rs"]
mod hook_rs_03_cargo_deny_step_present_tests;
