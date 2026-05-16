use guardrail3_check_types::G3CheckResult;

use crate::support::{BoundaryFieldSite, error};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-garde/context-validation-surface";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(field: &BoundaryFieldSite, results: &mut Vec<G3CheckResult>) {
    if !field.uses_context || field.boundary_has_context {
        return;
    }

    results.push(error(
        ID,
        format!(
            "boundary `{}` uses ctx without garde(context)",
            field.boundary_name
        ),
        format!(
            "Field `{}` in validated boundary `{}` references `ctx` in a garde validator, but the boundary type is missing `#[garde(context(...))]`. Add `#[garde(context(YourContextType))]` to the struct definition.",
            field.field_name, field.boundary_name
        ),
        &field.rel_path,
        Some(field.line),
    ));
}
