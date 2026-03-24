use crate::domain::report::{CheckResult, Severity};

use super::inputs::ToolDepsInput;

const ID: &str = "RS-DEPS-01";

pub fn check(input: &ToolDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.tool.tool_name != "cargo-deny" {
        return;
    }

    if input.tool.installed {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "cargo-deny installed".to_owned(),
                message: "`cargo-deny` is available on PATH.".to_owned(),
                file: None,
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "cargo-deny missing".to_owned(),
            message:
                "`cargo-deny` is required for Rust dependency guardrails but was not found on PATH."
                    .to_owned(),
            file: None,
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_deps_01_cargo_deny_installed_tests/mod.rs"]
mod tests;
