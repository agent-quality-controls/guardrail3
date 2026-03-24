use crate::domain::report::{CheckResult, Severity};

use super::inputs::ToolDepsInput;

const ID: &str = "RS-DEPS-03";

pub fn check(input: &ToolDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.tool.tool_name != "cargo-dupes" {
        return;
    }

    if input.tool.installed {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "cargo-dupes installed".to_owned(),
                message: "`cargo-dupes` is available on PATH.".to_owned(),
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
            title: "cargo-dupes missing".to_owned(),
            message:
                "`cargo-dupes` is recommended for Rust dependency guardrails but was not found on PATH."
                    .to_owned(),
            file: None,
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_deps_03_cargo_dupes_installed_tests/mod.rs"]
mod tests;
