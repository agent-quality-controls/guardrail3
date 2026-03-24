use crate::domain::report::{CheckResult, Severity};

use super::inputs::GardeRootInput;

const ID: &str = "RS-GARDE-01";

pub fn check(input: &GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    if input.root.garde_dependency_present {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "garde dependency found".to_owned(),
                message: format!(
                    "garde is present in `{}` for this {}. Garde-specific boundary checks are active.",
                    input.root.cargo_rel_path,
                    input.root.kind.label()
                ),
                file: Some(input.root.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "garde dependency missing".to_owned(),
            message: format!(
                "Missing `garde` dependency in `{}` for this {}. Runtime input validation at Rust adapter boundaries requires garde.",
                input.root.cargo_rel_path,
                input.root.kind.label()
            ),
            file: Some(input.root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_garde_01_dependency_present_tests/mod.rs"]
mod tests;
