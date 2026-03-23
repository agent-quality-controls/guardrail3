use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-08";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = input.parsed.executable_lines.iter().any(|line| {
        line.command_text.contains("guardrail3")
            && (line.command_text.contains("guardrail3 rs validate")
                || line.command_text.contains("guardrail3 validate"))
            && line.command_text.contains("--staged")
    });

    push_presence_result(
        found,
        input.rel_path,
        "Rust guardrail validate step present",
        "Hook runs guardrail3 Rust validation on staged changes.",
        "Rust guardrail validate step missing",
        "Hook does not execute `guardrail3 ... validate --staged`.",
        results,
    );
}

fn push_presence_result(
    found: bool,
    rel_path: &str,
    ok_title: &str,
    ok_message: &str,
    missing_title: &str,
    missing_message: &str,
    results: &mut Vec<CheckResult>,
) {
    if found {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: ok_title.to_owned(),
                message: ok_message.to_owned(),
                file: Some(rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: missing_title.to_owned(),
            message: missing_message.to_owned(),
            file: Some(rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "hook_rs_08_guardrail_validate_staged_present_tests.rs"]
mod tests;
