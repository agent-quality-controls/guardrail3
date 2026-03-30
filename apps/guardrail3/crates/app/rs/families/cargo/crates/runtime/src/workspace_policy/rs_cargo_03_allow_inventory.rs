use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PolicyRootCargoInput;
use super::lint_support::{EXPECTED_CLIPPY_ALLOW, lint_level, policy_lints};

const ID: &str = "RS-CARGO-03";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    let Some(_parsed) = root.parsed.as_ref() else {
        return;
    };
    let Some(clippy_lints) = policy_lints(root, "clippy") else {
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
                file: Some(root.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_cargo_03_allow_inventory_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_cargo_03_allow_inventory_tests;
