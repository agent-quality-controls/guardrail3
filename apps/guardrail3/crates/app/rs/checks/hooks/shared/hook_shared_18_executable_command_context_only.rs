use crate::domain::report::{CheckResult, Severity};

use super::facts::HookScriptKind;
use super::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-18";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    let mut suspicious_lines = Vec::new();

    for (index, raw_line) in input.content.lines().enumerate() {
        let line_no = index + 1;
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some(step) = suspicious_step(trimmed) else {
            continue;
        };
        let is_executable_match = input
            .parsed
            .executable_lines
            .iter()
            .any(|line| matches_step_family(line.command_text, step));
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
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "required hook step appears only in inert text".to_owned(),
            message: format!(
                "`{step}` appears in {kind} text but not on any executable command line."
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(line_no),
            inventory: false,
        });
    }
}

fn suspicious_step(line: &str) -> Option<&'static str> {
    if line.starts_with('#') || line.starts_with("echo ") {
        return step_family_from_text(line);
    }
    None
}

fn step_family_from_text(line: &str) -> Option<&'static str> {
    let families = [
        ("guardrail3 rs validate", "guardrail3 rs validate"),
        ("guardrail3 validate", "guardrail3 validate"),
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

fn matches_step_family(command_text: &str, family: &str) -> bool {
    match family {
        "guardrail3 rs validate" => command_text.contains("guardrail3 rs validate"),
        "guardrail3 validate" => command_text.contains("guardrail3 validate"),
        "cargo clippy" => command_text.contains("cargo clippy"),
        "cargo deny" => command_text.contains("cargo deny") || command_text.contains("cargo-deny"),
        "cargo test" => command_text.contains("cargo test"),
        "cargo machete" => {
            command_text.contains("cargo machete") || command_text.contains("cargo-machete")
        }
        "cargo dupes" => {
            command_text.contains("cargo dupes") || command_text.contains("cargo-dupes")
        }
        "gitleaks" => command_text.contains("gitleaks"),
        "pnpm install --frozen-lockfile" => command_text.contains("--frozen-lockfile"),
        _ => false,
    }
}

#[cfg(test)]
#[path = "hook_shared_18_executable_command_context_only_tests/mod.rs"]
mod tests;
