use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PolicyRootCargoInput;
use crate::lint_support::{
    has_valid_lint_level, lint_level, policy_lints, policy_lints_table_label,
};

const ID: &str = "RS-CARGO-11";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    let Some(_parsed) = root.parsed.as_ref() else {
        return;
    };
    let Some(clippy_lints) = policy_lints(root, "clippy") else {
        return;
    };
    let Some(clippy_table) = clippy_lints.as_table() else {
        return;
    };

    match clippy_table.get("disallowed_macros") {
        Some(value) if !has_valid_lint_level(value) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "disallowed macros lint invalid".to_owned(),
            format!(
                "`{}` must define `disallowed_macros` with a valid lint level and set it to `deny`.",
                policy_lints_table_label(root.kind, "clippy")
            ),
            Some(root.cargo_rel_path.clone()),
            None,
            false,
        )),
        _ => match lint_level(clippy_lints, "disallowed_macros").as_deref() {
        Some("deny") => {
            if root.guardrail_parse_error {
                return;
            }
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Info,
                    "disallowed macros lint enforced".to_owned(),
                    format!(
                        "`{}` enforces `clippy::disallowed_macros = \"deny\"`.",
                        policy_lints_table_label(root.kind, "clippy")
                    ),
                    Some(root.cargo_rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            )
        }
        Some(other) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "disallowed macros lint weakened".to_owned(),
            format!(
                "`{}` sets `disallowed_macros` to `{other}` instead of `deny`.",
                policy_lints_table_label(root.kind, "clippy")
            ),
            Some(root.cargo_rel_path.clone()),
            None,
            false,
        )),
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
        },
    }
}

#[cfg(test)]

// reason: test-only sidecar module wiring
mod tests;
