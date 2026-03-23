use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-01";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = input.parsed.executable_lines.iter().any(|line| {
        line.command_name == "cargo"
            && line.command_text.contains("cargo fmt")
            && line.command_text.contains("--check")
    });

    push_presence_result(
        found,
        input.rel_path,
        "cargo fmt --check step present",
        "Hook runs cargo fmt in check mode.",
        "cargo fmt --check step missing",
        "Hook does not execute `cargo fmt ... --check`.",
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
                severity: Severity::Warn,
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
#[path = "hook_rs_01_fmt_step_present_tests.rs"]
mod tests;
