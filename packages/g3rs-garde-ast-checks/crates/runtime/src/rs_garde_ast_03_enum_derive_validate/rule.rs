use guardrail3_check_types::G3CheckResult;

use crate::parse::BoundaryKind;
use crate::support::{DerivedBoundaryTypeSite, error};

const ID: &str = "RS-GARDE-AST-03";

pub(crate) fn check(target: &DerivedBoundaryTypeSite, results: &mut Vec<G3CheckResult>) {
    if target.boundary_kind != BoundaryKind::Enum || target.has_validate {
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

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
