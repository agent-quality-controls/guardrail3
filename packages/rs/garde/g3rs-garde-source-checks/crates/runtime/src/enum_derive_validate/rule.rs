use g3rs_garde_types::G3RsGardeBoundaryKind;
use guardrail3_check_types::G3CheckResult;

use crate::support::{DerivedBoundaryTypeSite, error};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-garde/enum-derive-validate";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(target: &DerivedBoundaryTypeSite, results: &mut Vec<G3CheckResult>) {
    if target.boundary_kind != G3RsGardeBoundaryKind::Enum || target.has_validate {
        return;
    }

    results.push(error(
        ID,
        format!("enum `{}` missing Validate derive", target.name),
        format!(
            "Enum `{}` derives {} and has non-primitive payload fields, but does not derive garde's `Validate`. Add `#[derive(Validate)]` to this enum.",
            target.name,
            target.boundary_macros.join(", ")
        ),
        &target.rel_path,
        Some(target.line),
    ));
}
