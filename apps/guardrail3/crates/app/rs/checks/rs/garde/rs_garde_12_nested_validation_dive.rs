use crate::domain::report::{CheckResult, Severity};

use super::inputs::BoundaryFieldInput;

const ID: &str = "RS-GARDE-12";

pub fn check(input: &BoundaryFieldInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.field.nested_validated || input.field.has_garde_dive {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!(
            "nested validated field `{}` missing garde(dive)",
            input.field.field_name
        ),
        message: format!(
            "Field `{}` in validated boundary `{}` points at validated nested type `{}` but is missing `#[garde(dive)]`. Nested validated fields must opt into recursive garde validation.",
            input.field.field_name, input.field.boundary_name, input.field.field_type
        ),
        file: Some(input.field.rel_path.clone()),
        line: Some(input.field.line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_garde_12_nested_validation_dive_tests/mod.rs"]
mod tests;
