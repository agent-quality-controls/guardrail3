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

#[cfg(test)]
fn check_input(input: &g3rs_garde_types::G3RsGardeSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for target in &input.enum_targets {
        check(target, &mut results);
    }
    results
}

#[cfg(test)]
struct Fixture(g3rs_garde_types::G3RsGardeSourceChecksInput);

#[cfg(test)]
impl Fixture {
    fn run(&self) -> Vec<G3CheckResult> {
        check_input(&self.0)
    }
}

#[cfg(test)]
fn fixture(enum_targets: Vec<crate::support::DerivedBoundaryTypeSite>) -> Fixture {
    let mut input = crate::support::active_source_input();
    input.enum_targets = enum_targets;
    Fixture(input)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
