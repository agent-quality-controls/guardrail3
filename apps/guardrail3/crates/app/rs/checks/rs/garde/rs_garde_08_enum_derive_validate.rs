use crate::domain::report::{CheckResult, Severity};

use super::inputs::DerivedBoundaryTypeInput;
use super::parse::BoundaryKind;

const ID: &str = "RS-GARDE-08";

pub fn check(input: &DerivedBoundaryTypeInput<'_>, results: &mut Vec<CheckResult>) {
    if input.target.boundary_kind != BoundaryKind::Enum || input.target.has_validate {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("enum `{}` missing Validate derive", input.target.name),
        message: format!(
            "Enum `{}` derives {} and has non-primitive payload fields, but does not derive `Validate`.",
            input.target.name,
            input.target.boundary_macros.join(", ")
        ),
        file: Some(input.target.rel_path.clone()),
        line: Some(input.target.line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_garde_08_enum_derive_validate_tests/mod.rs"]
mod tests;
