use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::BoundaryFieldInput;

const ID: &str = "RS-GARDE-13";

pub fn check(input: &BoundaryFieldInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.field.uses_context || input.field.boundary_has_context {
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!(
            "boundary `{}` uses ctx without garde(context)",
            input.field.boundary_name
        ),
    format!(
            "Field `{}` in validated boundary `{}` references `ctx` in a garde validator, but the boundary type is missing `#[garde(context(...))]`.",
            input.field.field_name, input.field.boundary_name
        ),
    Some(input.field.rel_path.clone()),
    Some(input.field.line),
    false,
    ));
}

