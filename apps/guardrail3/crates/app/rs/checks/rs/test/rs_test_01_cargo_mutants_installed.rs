use crate::domain::report::{CheckResult, Severity};

use super::inputs::ToolTestInput;

const ID: &str = "RS-TEST-01";

pub fn check(input: &ToolTestInput<'_>, results: &mut Vec<CheckResult>) {
    if input.tool.installed {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "cargo-mutants installed".to_owned(),
                message: "`cargo-mutants` is available on PATH.".to_owned(),
                file: None,
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "cargo-mutants missing".to_owned(),
            message: "`cargo-mutants` was not found on PATH.".to_owned(),
            file: None,
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_test_01_cargo_mutants_installed_tests.rs"]
mod tests;
