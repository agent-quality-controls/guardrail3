use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::DerivedBoundaryTypeInput;
use crate::parse::BoundaryKind;

const ID: &str = "RS-GARDE-05";

pub fn check(input: &DerivedBoundaryTypeInput<'_>, results: &mut Vec<CheckResult>) {
    if input.target.boundary_kind != BoundaryKind::Struct || input.target.has_validate {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!("struct `{}` missing Validate derive", input.target.name),
        format!(
            "Struct `{}` derives {} but does not derive garde's `Validate`. Add `#[derive(Validate)]` to this struct.",
            input.target.name,
            input.target.boundary_macros.join(", ")
        ),
        Some(input.target.rel_path.clone()),
        Some(input.target.line),
        false,
    ));
}

