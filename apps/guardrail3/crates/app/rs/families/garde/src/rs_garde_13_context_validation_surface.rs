use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::BoundaryFieldInput;

const ID: &str = "RS-GARDE-13";

pub fn check(input: &BoundaryFieldInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.field.uses_context || input.field.boundary_has_context {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!(
            "boundary `{}` uses ctx without garde(context)",
            input.field.boundary_name
        ),
        message: format!(
            "Field `{}` in validated boundary `{}` references `ctx` in a garde validator, but the boundary type is missing `#[garde(context(...))]`.",
            input.field.field_name, input.field.boundary_name
        ),
        file: Some(input.field.rel_path.clone()),
        line: Some(input.field.line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_garde_13_context_validation_surface_tests/mod.rs"]
mod tests;
