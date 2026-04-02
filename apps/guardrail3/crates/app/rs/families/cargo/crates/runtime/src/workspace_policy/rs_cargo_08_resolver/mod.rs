use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::PolicyRootKind;
use crate::inputs::PolicyRootCargoInput;

const ID: &str = "RS-CARGO-08";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    if root.kind != PolicyRootKind::WorkspaceRoot || root.parse_error.is_some() {
        return;
    }
    if root.resolver_invalid {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "workspace resolver invalid".to_owned(),
            "Every workspace root must set `resolver` to the string `\"2\"` or `\"3\"`.".to_owned(),
            Some(root.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    match root.resolver.as_deref() {
        Some("2" | "3") => {
            if root.guardrail_parse_error {
                return;
            }
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Info,
                    "workspace resolver set".to_owned(),
                    format!(
                        "Workspace resolver = `{}`",
                        root.resolver.as_deref().unwrap_or_default()
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
            "unsupported workspace resolver".to_owned(),
            format!("Expected resolver `2` or `3`, got `{other}`."),
            Some(root.cargo_rel_path.clone()),
            None,
            false,
        )),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "workspace resolver missing".to_owned(),
            message:
                "Every workspace root must set `resolver = \"2\"` or `resolver = \"3\"` explicitly."
                    .to_owned(),
            file: Some(root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]

mod tests;
