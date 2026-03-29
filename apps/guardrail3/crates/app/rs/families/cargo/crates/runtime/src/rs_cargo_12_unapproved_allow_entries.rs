use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PolicyRootCargoInput;
use super::lint_support::{explicit_allow_entries, is_approved_allow, policy_lints};

const ID: &str = "RS-CARGO-12";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    let Some(_parsed) = root.parsed.as_ref() else {
        return;
    };

    let mut violations = 0usize;
    for (family, lints) in [
        ("rust", policy_lints(root, "rust")),
        ("clippy", policy_lints(root, "clippy")),
    ] {
        for lint_name in explicit_allow_entries(lints) {
            if family == "clippy" && is_approved_allow(&lint_name) {
                continue;
            }
            violations += 1;
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "unapproved allow entry".to_owned(),
                message: format!(
                    "`{}` explicitly allows `{}` in `{family}` lint policy.",
                    root.cargo_rel_path, lint_name
                ),
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
                title: "no unapproved allow entries".to_owned(),
                message: format!(
                    "`{}` does not introduce manifest-level allow entries outside the approved inventory.",
                    root.cargo_rel_path
                ),
                file: Some(root.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_cargo_12_unapproved_allow_entries_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_cargo_12_unapproved_allow_entries_tests;
