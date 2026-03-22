use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceCargoInput;
use super::lint_support::{EXPECTED_CLIPPY_ALLOW, lint_level, workspace_lints};

const ID: &str = "RS-CARGO-03";

pub fn check(input: &WorkspaceCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.workspace.parsed.as_ref() else {
        if let Some(parse_error) = &input.workspace.parse_error {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "workspace Cargo.toml parse error".to_owned(),
                message: format!("Failed to parse workspace Cargo.toml: {parse_error}"),
                file: Some(input.workspace.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
        return;
    };

    let Some(clippy_lints) = workspace_lints(parsed, "clippy") else {
        return;
    };

    for lint_name in EXPECTED_CLIPPY_ALLOW {
        let message = match lint_level(clippy_lints, lint_name).as_deref() {
            Some("allow") => format!("`{lint_name}` is explicitly allowed."),
            Some(other) => format!("`{lint_name}` is set to `{other}` instead of `allow`."),
            None => format!("`{lint_name}` is not configured and falls back to group policy."),
        };

        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: format!("allow inventory: `{lint_name}`"),
                message,
                file: Some(input.workspace.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_cargo_03_allow_inventory_tests.rs"]
mod tests;
