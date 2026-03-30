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
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "edition policy satisfied".to_owned(),
                format!(
                    "`{}` declares edition `{}`.",
                    root.cargo_rel_path,
                    root.edition.as_deref().unwrap_or_default()
                ),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
        Some(other) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "edition below minimum".to_owned(),
            format!(
                "`{}` declares edition `{other}`. Cargo policy requires edition `2021` or newer.",
                root.cargo_rel_path
            ),
            Some(root.cargo_rel_path.clone()),
            None,
            false,
        )),
        None => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "edition missing".to_owned(),
            format!(
                "`{}` must declare an edition for this {}.",
                root.cargo_rel_path,
                root.kind.label()
            ),
            Some(root.cargo_rel_path.clone()),
            None,
            false,
        )),
    }
}

#[cfg(test)]
#[path = "rs_cargo_05_workspace_metadata_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_cargo_05_workspace_metadata_tests;
