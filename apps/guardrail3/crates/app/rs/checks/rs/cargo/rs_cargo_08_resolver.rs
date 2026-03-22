use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceCargoInput;

const ID: &str = "RS-CARGO-08";

pub fn check(input: &WorkspaceCargoInput<'_>, results: &mut Vec<CheckResult>) {
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
        return;
    }

    match input.workspace.resolver.as_deref() {
        Some("2" | "3") => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "workspace resolver set".to_owned(),
                message: format!(
                    "Workspace resolver = `{}`",
                    input.workspace.resolver.as_deref().unwrap_or_default()
                ),
                file: Some(input.workspace.rel_path.clone()),
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
            file: Some(input.workspace.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        None if input.workspace.has_package && has_modern_package_edition(input.workspace.workspace_edition.as_deref()) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "resolver omitted on non-virtual workspace".to_owned(),
                message: "Resolver is omitted, but this root package uses edition 2021+ so Cargo can infer a modern resolver.".to_owned(),
                file: Some(input.workspace.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        None if input.workspace.has_package => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "resolver omitted on pre-2021 non-virtual workspace".to_owned(),
            message: "Non-virtual workspaces without edition 2021+ must set `resolver = \"2\"` or `resolver = \"3\"` explicitly.".to_owned(),
            file: Some(input.workspace.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "virtual workspace missing resolver".to_owned(),
            message: "Virtual workspaces must set `resolver = \"2\"` or `resolver = \"3\"`.".to_owned(),
            file: Some(input.workspace.rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

fn has_modern_package_edition(edition: Option<&str>) -> bool {
    edition
        .and_then(|value| value.parse::<u32>().ok())
        .is_some_and(|edition| edition >= 2021)
}

#[cfg(test)]
#[path = "rs_cargo_08_resolver_tests.rs"]
mod tests;
