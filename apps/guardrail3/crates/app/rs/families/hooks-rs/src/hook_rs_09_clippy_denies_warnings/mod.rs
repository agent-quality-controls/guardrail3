mod support;

use guardrail3_app_rs_family_hooks_shared::hook_shell::{ParsedShellScript, parse_script};
use guardrail3_domain_report::{CheckResult, Severity};

use self::support::*;
use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-09";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct EnvState {
    rustflags: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct LintEffect {
    denied: bool,
    softened: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SegmentEvaluation {
    found: bool,
    persist_env: bool,
}

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = script_contains_clippy_deny(input.parsed);

    if found {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "cargo clippy denies warnings".to_owned(),
                "Hook runs clippy in a deny-warnings mode.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "cargo clippy deny-warnings step missing".to_owned(),
            "Hook does not execute `cargo clippy` with `-D warnings` or equivalent.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

fn script_contains_clippy_deny(parsed: &ParsedShellScript<'_>) -> bool {
    execute_script_for_clippy(parsed, parsed, &mut EnvState::default(), &mut Vec::new())
}

fn execute_script_for_clippy(
    parsed: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> bool {
    for line in parsed.executable_lines() {
        if line_contains_clippy_deny(line.raw(), root, env_state, visiting) {
            return true;
        }
    }

    false
}

fn line_contains_clippy_deny(
    raw: &str,
    root: &ParsedShellScript<'_>,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> bool {
    let segments = split_command_segments(raw);
    if segments.is_empty() {
        return segment_evaluation(raw, root, env_state, visiting).found;
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
            let mut segment_env = env_state.clone();
            let evaluation = segment_evaluation(&segment.text, root, &mut segment_env, visiting);
            if evaluation.found {
                return true;
            }
            if evaluation.persist_env {
                *env_state = segment_env;
            }

            for substitution in extract_command_substitutions(&segment.text) {
                let mut substitution_env = env_state.clone();
                if line_contains_clippy_deny(&substitution, root, &mut substitution_env, visiting) {
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

fn segment_evaluation(
    segment: &str,
    root: &ParsedShellScript<'_>,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> SegmentEvaluation {
    let tokens = shell_words(segment);
    let mut parts = TokenCursor::new(&tokens);
    let mut local_env = env_state.clone();
    let mut has_local_overlay = false;

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let token = parts.next().unwrap_or_default();
        apply_inline_assignment(token, &mut local_env);
        has_local_overlay = true;
    }

    let Some(first) = parts.next() else {
        return SegmentEvaluation {
            found: false,
            persist_env: false,
        };
    };

    match normalize_command_token(first) {
        "export" => {
            apply_export_assignments(&mut parts, env_state);
            SegmentEvaluation {
                found: false,
                persist_env: true,
            }
        }
        "unset" => {
            apply_unset_arguments(&mut parts, env_state);
            SegmentEvaluation {
                found: false,
                persist_env: true,
            }
        }
        "env" => SegmentEvaluation {
            found: env_wrapper_contains_clippy_deny(&mut parts, root, &mut local_env, visiting),
            persist_env: false,
        },
        "sh" | "bash" => SegmentEvaluation {
            found: shell_wrapper_contains_clippy_deny(&mut parts, root, &mut local_env, visiting),
            persist_env: false,
        },
        "command" => SegmentEvaluation {
            found: command_wrapper_contains_clippy_deny(&mut parts, root, &mut local_env, visiting),
            persist_env: false,
        },
        "exec" => SegmentEvaluation {
            found: exec_wrapper_contains_clippy_deny(&mut parts, root, &mut local_env, visiting),
            persist_env: false,
        },
        "cargo" => SegmentEvaluation {
            found: cargo_clippy_denies_warnings(&mut parts, &local_env),
            persist_env: false,
        },
        command_name => {
            if has_local_overlay {
                SegmentEvaluation {
                    found: called_function_contains_clippy_deny(
                        command_name,
                        root,
                        &mut local_env,
                        visiting,
                    ),
                    persist_env: false,
                }
            } else {
                SegmentEvaluation {
                    found: called_function_contains_clippy_deny(
                        command_name,
                        root,
                        env_state,
                        visiting,
                    ),
                    persist_env: true,
                }
            }
        }
    }
}

fn called_function_contains_clippy_deny(
    command_name: &str,
    root: &ParsedShellScript<'_>,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
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
    let found = execute_script_for_clippy(&body_parsed, root, env_state, visiting);
    let _ = visiting.pop();
    found
}

fn env_wrapper_contains_clippy_deny(
    parts: &mut TokenCursor<'_>,
    root: &ParsedShellScript<'_>,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> bool {
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
            match flag_name {
                "-u" | "--unset" => {
                    if value == "RUSTFLAGS" {
                        env_state.rustflags = None;
                    }
                }
                "-S" | "--split-string" => split_string = Some(value.to_owned()),
                _ => {}
            }
            continue;
        }
        if env_flag_takes_value(flag) {
            let value = parts.next().unwrap_or_default();
            match flag {
                "-u" | "--unset" if value == "RUSTFLAGS" => env_state.rustflags = None,
                "-S" | "--split-string" => split_string = Some(value.to_owned()),
                _ => {}
            }
        }
    }

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let token = parts.next().unwrap_or_default();
        apply_inline_assignment(token, env_state);
    }

    if let Some(script) = split_string {
        let mut nested = script;
        let tail = parts.remaining().join(" ");
        if !tail.is_empty() {
            nested.push(' ');
            nested.push_str(&tail);
        }
        return line_contains_clippy_deny(&nested, root, env_state, visiting);
    }

    let Some(next) = parts.next() else {
        return false;
    };

    wrapper_or_command_contains_clippy_deny(next, parts, root, env_state, visiting)
}

fn shell_wrapper_contains_clippy_deny(
    parts: &mut TokenCursor<'_>,
    root: &ParsedShellScript<'_>,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> bool {
    while let Some(token) = parts.peek() {
        if !token.starts_with('-') {
            break;
        }
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if let Some((flag_name, _)) = flag.split_once('=')
            && shell_flag_takes_value(flag_name)
        {
            continue;
        }
        if shell_flag_takes_value(flag) {
            let _ = parts.next();
        }
    }

    let Some(script) = parts.next() else {
        return false;
    };

    line_contains_clippy_deny(script, root, env_state, visiting)
}

fn command_wrapper_contains_clippy_deny(
    parts: &mut TokenCursor<'_>,
    root: &ParsedShellScript<'_>,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> bool {
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

    wrapper_or_command_contains_clippy_deny(next, parts, root, env_state, visiting)
}

fn exec_wrapper_contains_clippy_deny(
    parts: &mut TokenCursor<'_>,
    root: &ParsedShellScript<'_>,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> bool {
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

    wrapper_or_command_contains_clippy_deny(next, parts, root, env_state, visiting)
}

fn wrapper_or_command_contains_clippy_deny(
    token: &str,
    parts: &mut TokenCursor<'_>,
    root: &ParsedShellScript<'_>,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> bool {
    match normalize_command_token(token) {
        "cargo" => cargo_clippy_denies_warnings(parts, env_state),
        "sh" | "bash" => shell_wrapper_contains_clippy_deny(parts, root, env_state, visiting),
        "command" => command_wrapper_contains_clippy_deny(parts, root, env_state, visiting),
        "exec" => exec_wrapper_contains_clippy_deny(parts, root, env_state, visiting),
        "env" => env_wrapper_contains_clippy_deny(parts, root, env_state, visiting),
        command_name => {
            called_function_contains_clippy_deny(command_name, root, env_state, visiting)
        }
    }
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<CheckResult> {
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
mod tests;
