use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::{
    command_query::any_resolved_command, parse_script, types::ParsedShellScript,
};

use crate::inputs::RustHookCommandInput;

const ID: &str = "RS-HOOKS-SOURCE-25";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let coverage = evaluate_script(input.parsed, input.parsed, false, &mut Vec::new());
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
        return;
    }

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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Coverage {
    saw_cargo: bool,
    uncovered_cargo: bool,
    exported_target_dir: bool,
}

fn evaluate_script(
    parsed: &ParsedShellScript,
    root: &ParsedShellScript,
    exported_target_dir: bool,
    visiting: &mut Vec<String>,
) -> Coverage {
    let mut coverage = Coverage {
        exported_target_dir,
        ..Coverage::default()
    };

    for line in &parsed.executable_lines {
        if is_export_target_dir_line(&line.raw) {
            coverage.exported_target_dir = true;
            continue;
        }

        if is_unset_target_dir_line(&line.raw) {
            coverage.exported_target_dir = false;
            continue;
        }

        if line_has_direct_cargo(&line.raw) {
            coverage.saw_cargo = true;
            if !coverage.exported_target_dir && !line_has_inline_target_dir(&line.raw) {
                coverage.uncovered_cargo = true;
            }
            continue;
        }

        let Some(function_name) = called_function_name(&line.raw) else {
            continue;
        };
        let Some(function) = root
            .functions
            .iter()
            .find(|function| function.name == function_name)
        else {
            continue;
        };
        if visiting.iter().any(|name| name == &function.name) {
            continue;
        }

        visiting.push(function.name.clone());
        let body = parse_script(&function.body);
        let nested = evaluate_script(&body, root, coverage.exported_target_dir, visiting);
        let _ = visiting.pop();

        coverage.saw_cargo |= nested.saw_cargo;
        coverage.uncovered_cargo |= nested.uncovered_cargo;
        coverage.exported_target_dir = nested.exported_target_dir;
    }

    coverage
}

fn line_has_direct_cargo(raw: &str) -> bool {
    let parsed = parse_script(raw);
    any_resolved_command(&parsed, |command| command.command_name() == "cargo")
}

fn is_export_target_dir_line(raw: &str) -> bool {
    let tokens = shell_words(raw);
    if tokens.is_empty() {
        return false;
    }

    if tokens[0] == "export" {
        return tokens
            .iter()
            .skip(1)
            .any(|token| token.starts_with("CARGO_TARGET_DIR="));
    }

    false
}

fn is_unset_target_dir_line(raw: &str) -> bool {
    let tokens = shell_words(raw);
    tokens.first().is_some_and(|token| token == "unset")
        && tokens
            .iter()
            .skip(1)
            .any(|token| token == "CARGO_TARGET_DIR")
}

fn line_has_inline_target_dir(raw: &str) -> bool {
    let tokens = shell_words(raw);
    let mut saw_env = false;
    let mut saw_cargo = false;

    for token in tokens {
        if token == "env" {
            saw_env = true;
            continue;
        }
        if token.starts_with('-') && saw_env {
            continue;
        }
        if normalize_command_token(&token) == "cargo" {
            saw_cargo = true;
            continue;
        }
        if token.starts_with("CARGO_TARGET_DIR=") {
            return !saw_cargo;
        }
    }

    false
}

fn called_function_name(raw: &str) -> Option<String> {
    let tokens = shell_words(raw);
    let mut index = 0usize;

    while matches!(tokens.get(index), Some(token) if looks_like_env_assignment(token)) {
        index += 1;
    }

    let token = tokens.get(index)?;
    let token = normalize_command_token(token);
    if matches!(
        token,
        "cargo" | "env" | "export" | "unset" | "sh" | "bash" | "command" | "exec"
    ) {
        return None;
    }

    Some(token.to_owned())
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
