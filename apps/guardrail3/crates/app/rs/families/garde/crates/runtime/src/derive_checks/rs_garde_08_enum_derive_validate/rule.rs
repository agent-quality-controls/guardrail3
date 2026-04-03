use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::DerivedBoundaryTypeInput;
use crate::parse::BoundaryKind;

const ID: &str = "RS-GARDE-08";

pub fn check(input: &DerivedBoundaryTypeInput<'_>, results: &mut Vec<CheckResult>) {
    if input.target.boundary_kind != BoundaryKind::Enum || input.target.has_validate {
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!("enum `{}` missing Validate derive", input.target.name),
    format!(
            "Enum `{}` derives {} and has non-primitive payload fields, but does not derive garde's `Validate`. Add `#[derive(Validate)]` to this enum.",
            input.target.name,
            input.target.boundary_macros.join(", ")
        ),
    Some(input.target.rel_path.clone()),
    Some(input.target.line),
    false,
    ));
}

