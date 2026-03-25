use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::PolicyRootKind;
use super::inputs::PolicyRootCargoInput;

const ID: &str = "RS-CARGO-08";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    if root.kind != PolicyRootKind::WorkspaceRoot || root.parse_error.is_some() {
        return;
    }

    match root.resolver.as_deref() {
        Some("2" | "3") => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "workspace resolver set".to_owned(),
                message: format!(
                    "Workspace resolver = `{}`",
                    root.resolver.as_deref().unwrap_or_default()
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
            title: "unsupported workspace resolver".to_owned(),
            message: format!("Expected resolver `2` or `3`, got `{other}`."),
            file: Some(root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }),
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
#[path = "rs_cargo_08_resolver_tests/mod.rs"]
mod tests;
