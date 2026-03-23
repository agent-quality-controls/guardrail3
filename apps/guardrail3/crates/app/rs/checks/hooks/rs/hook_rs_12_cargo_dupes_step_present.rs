use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-12";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = input.parsed.executable_lines.iter().any(|line| {
        (line.command_name == "cargo" && line.command_text.contains("cargo dupes"))
            || line.command_name == "cargo-dupes"
    });

    if found {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "cargo-dupes step present".to_owned(),
                message: "Hook runs cargo-dupes as an executable command.".to_owned(),
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
            title: "cargo-dupes step missing".to_owned(),
            message: "Hook does not execute cargo-dupes.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "hook_rs_12_cargo_dupes_step_present_tests.rs"]
mod tests;
