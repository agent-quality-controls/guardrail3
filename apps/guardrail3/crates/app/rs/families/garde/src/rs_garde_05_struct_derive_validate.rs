use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::DerivedBoundaryTypeInput;
use super::parse::BoundaryKind;

const ID: &str = "RS-GARDE-05";

pub fn check(input: &DerivedBoundaryTypeInput<'_>, results: &mut Vec<CheckResult>) {
    if input.target.boundary_kind != BoundaryKind::Struct || input.target.has_validate {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("struct `{}` missing Validate derive", input.target.name),
        message: format!(
            "Struct `{}` derives {} but does not derive `Validate`. Non-primitive input boundary structs must derive garde validation.",
            input.target.name,
            input.target.boundary_macros.join(", ")
        ),
        file: Some(input.target.rel_path.clone()),
        line: Some(input.target.line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_garde_05_struct_derive_validate_tests/mod.rs"]
mod tests;
