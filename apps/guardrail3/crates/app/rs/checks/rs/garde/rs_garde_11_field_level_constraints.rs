use crate::domain::report::{CheckResult, Severity};

use super::inputs::BoundaryFieldInput;

const ID: &str = "RS-GARDE-11";

pub fn check(input: &BoundaryFieldInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.field.requires_field_validation
        || input.field.nested_validated
        || input.field.has_meaningful_garde_rule
    {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!(
            "boundary field `{}` missing garde validator",
            input.field.field_name
        ),
        message: format!(
            "Field `{}` in validated boundary `{}` has type `{}` but no meaningful garde validator. Add a field-level garde rule such as `length`, `range`, `url`, or another explicit validator.",
            input.field.field_name, input.field.boundary_name, input.field.field_type
        ),
        file: Some(input.field.rel_path.clone()),
        line: Some(input.field.line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_garde_11_field_level_constraints_tests/mod.rs"]
mod tests;
