use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-11";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = input
        .parsed
        .executable_lines
        .iter()
        .any(|line| line.command_name == "gitleaks");

    if found {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "gitleaks step present".to_owned(),
                message: "Hook runs gitleaks as an executable command.".to_owned(),
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
            title: "gitleaks step missing".to_owned(),
            message: "Hook does not execute gitleaks.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "hook_rs_11_gitleaks_step_present_tests.rs"]
mod tests;
