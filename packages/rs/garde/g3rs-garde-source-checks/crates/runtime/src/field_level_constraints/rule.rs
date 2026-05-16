use guardrail3_check_types::G3CheckResult;

use crate::support::{BoundaryFieldSite, error};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-garde/field-level-constraints";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(field: &BoundaryFieldSite, results: &mut Vec<G3CheckResult>) {
    if !field.requires_field_validation
        || field.has_garde_skip
        || field.nested_validated
        || field.has_garde_dive
        || field.has_meaningful_garde_rule
    {
        return;
    }

    results.push(error(
        ID,
        format!(
            "boundary field `{}` missing garde validator",
            field.field_name
        ),
        format!(
            "Field `{}` in validated boundary `{}` has type `{}` but no meaningful garde validator. Add a field-level garde rule such as `length`, `range`, `url`, or another explicit validator.",
            field.field_name, field.boundary_name, field.field_type
        ),
        &field.rel_path,
        Some(field.line),
    ));
}
