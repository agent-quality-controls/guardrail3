use crate::domain::report::{CheckResult, Severity};

use super::inputs::UnsafeCodeLintInput;

const ID: &str = "RS-CODE-12";

pub fn check(input: &UnsafeCodeLintInput<'_>, results: &mut Vec<CheckResult>) {
    match input.lint_level {
        Some("forbid") => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "unsafe_code = forbid".to_owned(),
                message: "unsafe_code is set to forbid in workspace lints.".to_owned(),
                file: Some(input.cargo_rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        Some("deny") => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "unsafe_code should be forbid".to_owned(),
            message: "unsafe_code = deny can be overridden; use forbid in workspace lints."
                .to_owned(),
            file: Some(input.cargo_rel_path.to_owned()),
            line: None,
            inventory: false,
        }),
        _ => {}
    }
}

#[cfg(test)]
#[path = "rs_code_12_unsafe_code_lint_tests/mod.rs"]
mod tests;
