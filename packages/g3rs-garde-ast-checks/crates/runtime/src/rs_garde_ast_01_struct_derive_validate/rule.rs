use guardrail3_check_types::G3CheckResult;

use crate::parse::BoundaryKind;
use crate::support::{DerivedBoundaryTypeSite, error};

const ID: &str = "RS-GARDE-AST-01";

pub(crate) fn check(target: &DerivedBoundaryTypeSite, results: &mut Vec<G3CheckResult>) {
    if target.boundary_kind != BoundaryKind::Struct || target.has_validate {
        return;
    }

    results.push(error(
        ID,
        format!("struct `{}` missing Validate derive", target.name),
        format!(
            "Struct `{}` derives {} but does not derive garde's `Validate`. Add `#[derive(Validate)]` to this struct.",
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
