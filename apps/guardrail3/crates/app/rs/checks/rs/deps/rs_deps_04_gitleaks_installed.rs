use crate::domain::report::{CheckResult, Severity};

use super::inputs::ToolDepsInput;

const ID: &str = "RS-DEPS-04";

pub fn check(input: &ToolDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.tool.tool_name != "gitleaks" {
        return;
    }

    if input.tool.installed {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "gitleaks installed".to_owned(),
                message: "`gitleaks` is available on PATH.".to_owned(),
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
            title: "gitleaks missing".to_owned(),
            message:
                "`gitleaks` is required for Rust dependency guardrails but was not found on PATH."
                    .to_owned(),
            file: None,
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_deps_04_gitleaks_installed_tests.rs"]
mod tests;
