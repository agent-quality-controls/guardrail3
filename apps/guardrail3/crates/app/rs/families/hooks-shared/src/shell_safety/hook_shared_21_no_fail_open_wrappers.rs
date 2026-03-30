use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::FailOpenWrapperInput;

const ID: &str = "HOOK-SHARED-21";

pub fn check(input: &FailOpenWrapperInput<'_>, results: &mut Vec<CheckResult>) {
    for line in input.executable_lines {
        if line.softened_by().is_none() || !is_guardrail_critical(line.command_text()) {
            continue;
        }

        results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Warn,
    "critical hook command is fail-open".to_owned(),
    format!(
                "Critical hook command `{}` is softened by a fail-open wrapper.",
                line.command_text()
            ),
    Some(input.rel_path.to_owned()),
    Some(line.line_no()),
    false,
        ));
    }

fn is_guardrail_critical(command_text: &str) -> bool {
    let command_name = command_text.split_whitespace().next().unwrap_or_default();
    command_name == "guardrail3"
        || command_name == "gitleaks"
        || command_name == "cargo-deny"
        || command_name == "cargo-machete"
        || command_name == "cargo-dupes"
        || (command_name == "cargo"
            && (command_text.contains("cargo clippy")
                || command_text.contains("cargo deny")
                || command_text.contains("cargo test")
                || command_text.contains("cargo machete")
                || command_text.contains("cargo dupes")))
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<CheckResult> {
    let parsed = crate::hook_shell::parse_script(content);
    let input = FailOpenWrapperInput {
        rel_path: ".githooks/pre-commit",
        executable_lines: parsed.executable_lines(),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "hook_shared_21_no_fail_open_wrappers_tests/mod.rs"]
mod hook_shared_21_no_fail_open_wrappers_tests;
