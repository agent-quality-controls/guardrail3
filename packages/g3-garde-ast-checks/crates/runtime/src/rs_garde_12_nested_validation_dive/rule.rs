use guardrail3_check_types::G3CheckResult;

use crate::support::{BoundaryFieldSite, error};

const ID: &str = "RS-GARDE-12";

pub(crate) fn check(field: &BoundaryFieldSite, results: &mut Vec<G3CheckResult>) {
    if !field.nested_validated || field.has_garde_skip || field.has_garde_dive {
        return;
    }

    results.push(error(
        ID,
        format!(
            "nested validated field `{}` missing garde(dive)",
            field.field_name
        ),
        format!(
            "Field `{}` in validated boundary `{}` points at validated nested type `{}` but is missing `#[garde(dive)]`. Nested validated fields must opt into recursive garde validation.",
            field.field_name, field.boundary_name, field.field_type
        ),
        &field.rel_path,
        Some(field.line),
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
