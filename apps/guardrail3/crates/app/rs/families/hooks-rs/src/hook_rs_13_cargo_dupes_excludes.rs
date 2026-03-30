mod support;

use guardrail3_app_rs_family_hooks_shared::hook_shell::{ParsedShellScript, parse_script};
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;
use self::support::*;
use self::support::helpers::{
    constant_exit_status, extract_command_substitutions, is_terminal_exit,
    looks_like_env_assignment, normalize_command_token, shell_words,
    split_command_segments, token_is_shadowed_function,
};

const ID: &str = "HOOK-RS-13";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = script_contains_cargo_dupes_with_exclude_tests(input.parsed);

    if found {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "cargo-dupes excludes tests".to_owned(),
                message: "Hook runs cargo-dupes with `--exclude-tests`.".to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "cargo-dupes exclude-tests flag missing".to_owned(),
            message: "Hook does not execute cargo-dupes with `--exclude-tests`.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

fn script_contains_cargo_dupes_with_exclude_tests(parsed: &ParsedShellScript<'_>) -> bool {
    script_contains_cargo_dupes(parsed, true) && !script_contains_cargo_dupes(parsed, false)
}

fn script_contains_cargo_dupes(parsed: &ParsedShellScript<'_>, want_exclude_tests: bool) -> bool {
    parsed.executable_lines.iter().any(|line| {
        let mut visiting = Vec::new();
        segment_contains_cargo_dupes(
            line.command_text,
            parsed,
            parsed,
            &mut visiting,
            want_exclude_tests,
            line.line_no,
            line.line_no,
        ) || line_contains_cargo_dupes(
            line.raw,
            parsed,
            parsed,
            &mut visiting,
            want_exclude_tests,
            line.line_no,
            line.line_no,
        )
    })
}

fn line_contains_cargo_dupes(
    raw: &str,
    current: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
    want_exclude_tests: bool,
    current_cutoff: usize,
    root_cutoff: usize,
) -> bool {
    let segments = split_command_segments(raw);
    if segments.is_empty() {
        return segment_contains_cargo_dupes(
            raw,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        );
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

        if reachable {
            if segment_contains_cargo_dupes(
                &segment.text,
                current,
                root,
                visiting,
                want_exclude_tests,
                current_cutoff,
                root_cutoff,
            ) {
                return true;
            }
            for substitution in extract_command_substitutions(&segment.text) {
                if line_contains_cargo_dupes(
                    &substitution,
                    current,
                    root,
                    visiting,
                    want_exclude_tests,
                    current_cutoff,
                    root_cutoff,
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

fn segment_contains_cargo_dupes(
    segment: &str,
    current: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
    want_exclude_tests: bool,
    current_cutoff: usize,
    root_cutoff: usize,
) -> bool {
    let tokens = shell_words(segment);
    let mut parts = tokens.iter().map(String::as_str).peekable();

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    let Some(first) = parts.next() else {
        return false;
    };

    let command_name = normalize_command_token(first);
    if token_is_shadowed_function(
        first,
        command_name,
        current,
        root,
        current_cutoff,
        root_cutoff,
    ) {
        return called_function_contains_cargo_dupes(
            command_name,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        );
    }

    match command_name {
        "env" => env_wrapper_contains_cargo_dupes(
            parts,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
        "sh" | "bash" => shell_wrapper_contains_cargo_dupes(
            parts,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
        "command" => command_wrapper_contains_cargo_dupes(
            parts,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
        "exec" => exec_wrapper_contains_cargo_dupes(
            parts,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
        "cargo" => cargo_dupes_subcommand_invocation(&mut parts, want_exclude_tests),
        "cargo-dupes" => cargo_dupes_binary_invocation(&mut parts, want_exclude_tests),
        command_name => called_function_contains_cargo_dupes(
            command_name,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
    }
}

fn called_function_contains_cargo_dupes(
    command_name: &str,
    current: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
    want_exclude_tests: bool,
    current_cutoff: usize,
    root_cutoff: usize,
) -> bool {
    let Some(function) = current
        .functions
        .iter()
        .find(|function| function.name == command_name && function.line_no <= current_cutoff)
        .or_else(|| {
            root.functions
                .iter()
                .find(|function| function.name == command_name && function.line_no <= root_cutoff)
        })
    else {
        return false;
    };
    if visiting.iter().any(|name| name == &function.name) {
        return false;
    }

    visiting.push(function.name.clone());
    let body_parsed = parse_script(&function.body);
    let found = body_parsed.executable_lines.iter().any(|line| {
        line_contains_cargo_dupes(
            line.raw,
            &body_parsed,
            root,
            visiting,
            want_exclude_tests,
            line.line_no,
            root_cutoff,
        )
    });
    let _ = visiting.pop();
    found
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
#[path = "hook_rs_13_cargo_dupes_excludes_tests/mod.rs"]
mod hook_rs_13_cargo_dupes_excludes_tests;
