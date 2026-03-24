use crate::domain::report::{CheckResult, Severity};

use super::inputs::PolicyRootCargoInput;
use super::lint_support::{EXPECTED_CLIPPY_DENY, lint_priority, policy_lints};

const ID: &str = "RS-CARGO-07";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    let Some(_parsed) = root.parsed.as_ref() else {
        return;
    };
    let Some(clippy_lints) = policy_lints(root, "clippy") else {
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
                file: Some(root.cargo_rel_path.clone()),
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
                file: Some(root.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_cargo_07_priority_order_tests/mod.rs"]
mod tests;
