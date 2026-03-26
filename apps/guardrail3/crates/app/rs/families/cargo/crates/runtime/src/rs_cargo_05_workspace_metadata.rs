use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PolicyRootCargoInput;

const ID: &str = "RS-CARGO-05";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    if root.parse_error.is_some() {
        return;
    }

    match root.edition.as_deref() {
        Some("2024" | "2021") => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "edition policy satisfied".to_owned(),
                message: format!(
                    "`{}` declares edition `{}`.",
                    root.cargo_rel_path,
                    root.edition.as_deref().unwrap_or_default()
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
            title: "edition below minimum".to_owned(),
            message: format!(
                "`{}` declares edition `{other}`. Cargo policy requires edition `2021` or newer.",
                root.cargo_rel_path
            ),
            file: Some(root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "edition missing".to_owned(),
            message: format!(
                "`{}` must declare an edition for this {}.",
                root.cargo_rel_path,
                root.kind.label()
            ),
            file: Some(root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
#[path = "rs_cargo_05_workspace_metadata_tests/mod.rs"]
mod rs_cargo_05_workspace_metadata_tests;
