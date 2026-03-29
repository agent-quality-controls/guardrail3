use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PolicyRootCargoInput;
use super::lint_support::{lint_level, policy_lints, policy_lints_table_label};

const ID: &str = "RS-CARGO-11";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    let Some(_parsed) = root.parsed.as_ref() else {
        return;
    };
    let Some(clippy_lints) = policy_lints(root, "clippy") else {
        return;
    };

    match lint_level(clippy_lints, "disallowed_macros").as_deref() {
        Some("deny") => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "disallowed macros lint enforced".to_owned(),
                message: format!(
                    "`{}` enforces `clippy::disallowed_macros = \"deny\"`.",
                    policy_lints_table_label(root.kind, "clippy")
                ),
                file: Some(root.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        Some(other) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "disallowed macros lint weakened".to_owned(),
            message: format!(
                "`{}` sets `disallowed_macros` to `{other}` instead of `deny`.",
                policy_lints_table_label(root.kind, "clippy")
            ),
            file: Some(root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "disallowed macros lint missing".to_owned(),
            message: format!(
                "`{}` must define `disallowed_macros = \"deny\"` so macro bans are enforceable.",
                policy_lints_table_label(root.kind, "clippy")
            ),
            file: Some(root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
#[path = "rs_cargo_11_disallowed_macros_deny_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_cargo_11_disallowed_macros_deny_tests;
