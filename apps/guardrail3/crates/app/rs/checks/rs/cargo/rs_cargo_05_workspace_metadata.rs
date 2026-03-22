use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceCargoInput;

const ID: &str = "RS-CARGO-05";

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

    let mut metadata_parts = Vec::new();
    if let Some(edition) = &input.workspace.workspace_edition {
        metadata_parts.push(format!("edition = {edition}"));
    }
    if let Some(rust_version) = &input.workspace.workspace_rust_version {
        metadata_parts.push(format!("rust-version = {rust_version}"));
    }

    if !metadata_parts.is_empty() {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "workspace metadata".to_owned(),
                message: format!("Workspace metadata: {}", metadata_parts.join(", ")),
                file: Some(input.workspace.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }

    match input.workspace.workspace_edition.as_deref() {
        Some("2024" | "2021") => {}
        Some(other) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "outdated workspace edition".to_owned(),
            message: format!(
                "Workspace edition is `{other}`. Use edition `2024` or `2021` minimum."
            ),
            file: Some(input.workspace.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "workspace edition missing".to_owned(),
            message: "Set `edition` in `[workspace.package]` or `[package]`.".to_owned(),
            file: Some(input.workspace.rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}
