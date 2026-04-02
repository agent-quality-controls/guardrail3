use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::BoundaryFieldInput;

const ID: &str = "RS-GARDE-12";

pub fn check(input: &BoundaryFieldInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.field.nested_validated || input.field.has_garde_skip || input.field.has_garde_dive {
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!(
            "nested validated field `{}` missing garde(dive)",
            input.field.field_name
        ),
    format!(
            "Field `{}` in validated boundary `{}` points at validated nested type `{}` but is missing `#[garde(dive)]`. Nested validated fields must opt into recursive garde validation.",
            input.field.field_name, input.field.boundary_name, input.field.field_type
        ),
    Some(input.field.rel_path.clone()),
    Some(input.field.line),
    false,
    ));
}

