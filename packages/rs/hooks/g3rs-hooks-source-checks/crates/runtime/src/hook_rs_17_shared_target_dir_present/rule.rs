use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::{parse_script, types::ParsedShellScript};

use super::support::*;
use crate::inputs::RustHookCommandInput;

const ID: &str = "RS-HOOKS-SOURCE-25";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct EnvState {
    pub(super) target_dir: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Coverage {
    saw_cargo: bool,
    uncovered_cargo: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SegmentEvaluation {
    coverage: Coverage,
    persist_env: bool,
}

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let coverage = script_coverage(input.parsed);
    if !coverage.saw_cargo {
        return;
    }

    if coverage.uncovered_cargo {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "shared CARGO_TARGET_DIR missing".to_owned(),
            "Hook runs cargo without a shared `CARGO_TARGET_DIR`. In a monorepo with multiple Cargo workspaces, each workspace falls back to its own `target/` directory and recompiles the same dependencies, proc-macros, and shared path dependencies separately. Set a repo-local shared target dir before cargo commands, for example `export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"`.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    } else {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "shared CARGO_TARGET_DIR configured".to_owned(),
                "Hook sets `CARGO_TARGET_DIR` for cargo execution. This reuses one repo-local build cache across Cargo workspaces and cuts duplicate dependency and proc-macro rebuilds during hook runs.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
    }
}

fn script_coverage(parsed: &ParsedShellScript) -> Coverage {
    execute_script_for_target_dir(parsed, parsed, &mut EnvState::default(), &mut Vec::new())
}

fn execute_script_for_target_dir(
    parsed: &ParsedShellScript,
    root: &ParsedShellScript,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> Coverage {
    let mut coverage = Coverage::default();

    for line in &parsed.executable_lines {
        let line_coverage = line_target_dir_coverage(&line.raw, root, env_state, visiting);
        coverage.saw_cargo |= line_coverage.saw_cargo;
        coverage.uncovered_cargo |= line_coverage.uncovered_cargo;
        if coverage.uncovered_cargo {
            return coverage;
        }
    }

    coverage
}

fn line_target_dir_coverage(
    raw: &str,
    root: &ParsedShellScript,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> Coverage {
    let segments = split_command_segments(raw);
    if segments.is_empty() {
        return segment_evaluation(raw, root, env_state, visiting).coverage;
    }

    let mut coverage = Coverage::default();
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
            let mut segment_env = env_state.clone();
            let evaluation = segment_evaluation(&segment.text, root, &mut segment_env, visiting);
            coverage.saw_cargo |= evaluation.coverage.saw_cargo;
            coverage.uncovered_cargo |= evaluation.coverage.uncovered_cargo;
            if evaluation.persist_env {
                *env_state = segment_env;
            }

            for substitution in extract_command_substitutions(&segment.text) {
                let mut substitution_env = env_state.clone();
                let substitution_coverage =
                    line_target_dir_coverage(&substitution, root, &mut substitution_env, visiting);
                coverage.saw_cargo |= substitution_coverage.saw_cargo;
                coverage.uncovered_cargo |= substitution_coverage.uncovered_cargo;
            }

            if coverage.uncovered_cargo {
                return coverage;
            }
        }

        if reachable {
            prefix_status = constant_exit_status(&segment.text);
        }
    }

    coverage
}

fn segment_evaluation(
    segment: &str,
    root: &ParsedShellScript,
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
            coverage: Coverage::default(),
            persist_env: false,
        };
    };

    let command_name = normalize_command_token(first);

    match command_name {
        "export" => {
            apply_export_assignments(&mut parts, env_state);
            SegmentEvaluation {
                coverage: Coverage::default(),
                persist_env: true,
            }
        }
        "unset" => {
            apply_unset_arguments(&mut parts, env_state);
            SegmentEvaluation {
                coverage: Coverage::default(),
                persist_env: true,
            }
        }
        "env" => SegmentEvaluation {
            coverage: env_wrapper_target_dir_coverage(&mut parts, root, &mut local_env, visiting),
            persist_env: false,
        },
        "sh" | "bash" => SegmentEvaluation {
            coverage: shell_wrapper_target_dir_coverage(&mut parts, root, &mut local_env, visiting),
            persist_env: false,
        },
        "command" => SegmentEvaluation {
            coverage: command_wrapper_target_dir_coverage(
                &mut parts,
                root,
                &mut local_env,
                visiting,
            ),
            persist_env: false,
        },
        "exec" => SegmentEvaluation {
            coverage: exec_wrapper_target_dir_coverage(&mut parts, root, &mut local_env, visiting),
            persist_env: false,
        },
        _ if root
            .functions
            .iter()
            .any(|function| function.name == command_name) =>
        {
            if has_local_overlay {
                SegmentEvaluation {
                    coverage: called_function_target_dir_coverage(
                        command_name,
                        root,
                        &mut local_env,
                        visiting,
                    ),
                    persist_env: false,
                }
            } else {
                SegmentEvaluation {
                    coverage: called_function_target_dir_coverage(
                        command_name,
                        root,
                        env_state,
                        visiting,
                    ),
                    persist_env: true,
                }
            }
        }
        "cargo" => SegmentEvaluation {
            coverage: Coverage {
                saw_cargo: true,
                uncovered_cargo: !local_env.target_dir,
            },
            persist_env: false,
        },
        _ => SegmentEvaluation {
            coverage: Coverage::default(),
            persist_env: false,
        },
    }
}

fn called_function_target_dir_coverage(
    command_name: &str,
    root: &ParsedShellScript,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> Coverage {
    let Some(function) = root
        .functions
        .iter()
        .find(|function| function.name == command_name)
    else {
        return Coverage::default();
    };
    if visiting.iter().any(|name| name == &function.name) {
        return Coverage::default();
    }

    visiting.push(function.name.to_owned());
    let body_parsed = parse_script(&function.body);
    let coverage = execute_script_for_target_dir(&body_parsed, root, env_state, visiting);
    let _ = visiting.pop();
    coverage
}

fn env_wrapper_target_dir_coverage(
    parts: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> Coverage {
    let mut split_string = None;

    while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return Coverage::default();
        }
        if flag == "--" {
            break;
        }
        if let Some((flag_name, value)) = flag.split_once('=')
            && env_flag_takes_value(flag_name)
        {
            match flag_name {
                "-u" | "--unset" if value == "CARGO_TARGET_DIR" => env_state.target_dir = false,
                "-S" | "--split-string" => split_string = Some(value.to_owned()),
                _ => {}
            }
            continue;
        }
        if env_flag_without_value(flag) {
            continue;
        }
        if env_flag_takes_value(flag) {
            let value = parts.next().unwrap_or_default();
            match flag {
                "-u" | "--unset" if value == "CARGO_TARGET_DIR" => env_state.target_dir = false,
                "-S" | "--split-string" => split_string = Some(value.to_owned()),
                _ => {}
            }
            continue;
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
        return line_target_dir_coverage(&nested, root, env_state, visiting);
    }

    let Some(next) = parts.next() else {
        return Coverage::default();
    };

    wrapper_or_command_target_dir_coverage(next, parts, root, env_state, visiting)
}

fn shell_wrapper_target_dir_coverage(
    parts: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> Coverage {
    while let Some(token) = parts.peek() {
        if !token.starts_with('-') {
            break;
        }
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return Coverage::default();
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
        return Coverage::default();
    };

    line_target_dir_coverage(script, root, env_state, visiting)
}

fn command_wrapper_target_dir_coverage(
    parts: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> Coverage {
    while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) || matches!(flag, "-v" | "-V") {
            return Coverage::default();
        }
        if flag == "--" {
            break;
        }
        if flag != "-p" {
            return Coverage::default();
        }
    }

    let Some(next) = parts.next() else {
        return Coverage::default();
    };

    wrapper_or_command_target_dir_coverage(next, parts, root, env_state, visiting)
}

fn exec_wrapper_target_dir_coverage(
    parts: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> Coverage {
    while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return Coverage::default();
        }
        if flag == "--" {
            break;
        }
    }

    let Some(next) = parts.next() else {
        return Coverage::default();
    };

    wrapper_or_command_target_dir_coverage(next, parts, root, env_state, visiting)
}

fn wrapper_or_command_target_dir_coverage(
    token: &str,
    parts: &mut TokenCursor<'_>,
    root: &ParsedShellScript,
    env_state: &mut EnvState,
    visiting: &mut Vec<String>,
) -> Coverage {
    match normalize_command_token(token) {
        "cargo" => Coverage {
            saw_cargo: true,
            uncovered_cargo: !env_state.target_dir,
        },
        "sh" | "bash" => shell_wrapper_target_dir_coverage(parts, root, env_state, visiting),
        "command" => command_wrapper_target_dir_coverage(parts, root, env_state, visiting),
        "exec" => exec_wrapper_target_dir_coverage(parts, root, env_state, visiting),
        "env" => env_wrapper_target_dir_coverage(parts, root, env_state, visiting),
        command_name => {
            called_function_target_dir_coverage(command_name, root, env_state, visiting)
        }
    }
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = parse_script(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        is_workspace_project: true,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
