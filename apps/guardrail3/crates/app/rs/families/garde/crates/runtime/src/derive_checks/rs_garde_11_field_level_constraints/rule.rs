use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::BoundaryFieldInput;

const ID: &str = "RS-GARDE-11";

pub fn check(input: &BoundaryFieldInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.field.requires_field_validation
        || input.field.has_garde_skip
        || input.field.nested_validated
        || input.field.has_garde_dive
        || input.field.has_meaningful_garde_rule
    {
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!(
            "boundary field `{}` missing garde validator",
            input.field.field_name
        ),
    format!(
            "Field `{}` in validated boundary `{}` has type `{}` but no meaningful garde validator. Add a field-level garde rule such as `length`, `range`, `url`, or another explicit validator.",
            input.field.field_name, input.field.boundary_name, input.field.field_type
        ),
    Some(input.field.rel_path.clone()),
    Some(input.field.line),
    false,
    ));
}

