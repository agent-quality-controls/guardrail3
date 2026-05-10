#![expect(
    clippy::type_complexity,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command_relaxed};

use crate::facts::HookScriptKind;
use crate::inputs::ExecutableCommandContextInput;
use crate::support::cargo_subcommand_tail;

/// `ID` constant.
const ID: &str = "g3rs-hooks/executable-command-context-only";

/// `check` function.
pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    let mut suspicious_lines = Vec::new();

    for logical_line in &input.parsed.source_lines {
        let line_no = logical_line.line_no;
        let trimmed = logical_line.raw.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some(step) = suspicious_step(trimmed) else {
            continue;
        };
        let is_executable_match = matches_step_family(input.parsed, step);
        if !is_executable_match {
            suspicious_lines.push((line_no, step));
        }
    }

    if suspicious_lines.is_empty() {
        return;
    }

    let kind = match input.kind {
        HookScriptKind::PreCommit => "pre-commit",
        HookScriptKind::Modular => "modular hook script",
    };
    for (line_no, step) in suspicious_lines {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Error,
            "required hook step appears only in inert text".to_owned(),
            format!("`{step}` appears in {kind} text but not on any executable command line."),
            Some(input.rel_path.to_owned()),
            Some(line_no),
            false,
        ));
    }
}

/// `suspicious_step` function.
fn suspicious_step(line: &str) -> Option<&'static str> {
    if line.starts_with("#!") {
        return None;
    }

    if let Some(comment) = crate::support::inline_comment_text(line) {
        return step_family_from_text(comment);
    }

    step_family_from_text(line)
}

/// `step_family_from_text` function.
fn step_family_from_text(line: &str) -> Option<&'static str> {
    let families = [
        ("g3rs rs validate", "g3rs rs validate"),
        ("g3rs validate", "g3rs validate"),
        ("cargo clippy", "cargo clippy"),
        ("cargo deny", "cargo deny"),
        ("cargo-deny", "cargo deny"),
        ("cargo test", "cargo test"),
        ("cargo machete", "cargo machete"),
        ("cargo-machete", "cargo machete"),
        ("cargo dupes", "cargo dupes"),
        ("cargo-dupes", "cargo dupes"),
        ("gitleaks", "gitleaks"),
        ("--frozen-lockfile", "pnpm install --frozen-lockfile"),
    ];
    families
        .into_iter()
        .find_map(|(needle, family)| line.contains(needle).then_some(family))
}

/// `matches_step_family` function.
fn matches_step_family(parsed: &hook_shell_parser::types::ParsedShellScript, family: &str) -> bool {
    any_resolved_command_relaxed(parsed, predicate_for_step_family(family))
}

/// `predicate_for_step_family` function.
fn predicate_for_step_family(family: &str) -> fn(&ResolvedCommand) -> bool {
    match family {
        "g3rs rs validate" => is_guardrail_rs_validate_command,
        "g3rs validate" => is_guardrail_validate_command,
        "cargo clippy" => is_cargo_clippy_command,
        "cargo deny" => is_cargo_deny_command,
        "cargo test" => is_cargo_test_command,
        "cargo machete" => is_cargo_machete_command,
        "cargo dupes" => is_cargo_dupes_command,
        "gitleaks" => is_gitleaks_command,
        "pnpm install --frozen-lockfile" => is_frozen_lockfile_command,
        _ => |_| false,
    }
}

/// `is_guardrail_rs_validate_command` function.
fn is_guardrail_rs_validate_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "g3rs"
        && matches!(command.args(), [first, second, ..] if first == "rs" && second == "validate")
}

/// `is_guardrail_validate_command` function.
fn is_guardrail_validate_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "g3rs" && matches!(command.args(), [first, ..] if first == "validate")
}

/// `is_cargo_clippy_command` function.
fn is_cargo_clippy_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "cargo" && cargo_subcommand_tail(command, "clippy").is_some()
}

/// `is_cargo_deny_command` function.
fn is_cargo_deny_command(command: &ResolvedCommand) -> bool {
    (command.command_name() == "cargo" && cargo_subcommand_tail(command, "deny").is_some())
        || command.command_name() == "cargo-deny"
}

/// `is_cargo_test_command` function.
fn is_cargo_test_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "cargo" && cargo_subcommand_tail(command, "test").is_some()
}

/// `is_cargo_machete_command` function.
fn is_cargo_machete_command(command: &ResolvedCommand) -> bool {
    (command.command_name() == "cargo" && cargo_subcommand_tail(command, "machete").is_some())
        || command.command_name() == "cargo-machete"
}

/// `is_cargo_dupes_command` function.
fn is_cargo_dupes_command(command: &ResolvedCommand) -> bool {
    (command.command_name() == "cargo" && cargo_subcommand_tail(command, "dupes").is_some())
        || command.command_name() == "cargo-dupes"
}

/// `is_gitleaks_command` function.
fn is_gitleaks_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "gitleaks"
}

/// `is_frozen_lockfile_command` function.
fn is_frozen_lockfile_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "pnpm"
        && matches!(
            command.args().first().map(String::as_str),
            Some("install" | "i")
        )
        && command.args().iter().any(|arg| arg == "--frozen-lockfile")
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: crate::facts::HookScriptKind::PreCommit,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
