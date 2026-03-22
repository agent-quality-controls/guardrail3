use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceCargoInput;
use super::lint_support::{EXPECTED_CLIPPY_DENY, lint_priority, workspace_lints};

const ID: &str = "RS-CARGO-07";

pub fn check(input: &WorkspaceCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.workspace.parsed.as_ref() else {
        return;
    };
    let Some(clippy_lints) = workspace_lints(parsed, "clippy") else {
        return;
    };

    let mut violations = 0usize;
    for lint_name in EXPECTED_CLIPPY_DENY {
        if lint_priority(clippy_lints, lint_name).is_some_and(|priority| priority < 0) {
            violations += 1;
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: format!("specific lint `{lint_name}` has negative priority"),
                message: "Specific clippy denies should keep default priority so groups do not override them."
                    .to_owned(),
                file: Some(input.workspace.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }

    if violations == 0 {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "specific lint priorities are safe".to_owned(),
                message: "Specific clippy deny lints do not use negative priority.".to_owned(),
                file: Some(input.workspace.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_cargo_07_priority_order_tests.rs"]
mod tests;
