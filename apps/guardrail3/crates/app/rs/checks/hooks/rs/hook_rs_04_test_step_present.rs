use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-04";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = input
        .parsed
        .executable_lines
        .iter()
        .any(|line| line.command_name == "cargo" && line.command_text.contains("cargo test"));

    if found {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "cargo test step present".to_owned(),
                message: "Hook runs cargo test.".to_owned(),
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
            title: "cargo test step missing".to_owned(),
            message: "Hook does not execute cargo test.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "hook_rs_04_test_step_present_tests.rs"]
mod tests;
