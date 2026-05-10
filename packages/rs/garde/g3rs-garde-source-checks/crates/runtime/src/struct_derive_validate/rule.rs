use g3rs_garde_types::G3RsGardeBoundaryKind;
use guardrail3_check_types::G3CheckResult;

use crate::support::{DerivedBoundaryTypeSite, error};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-garde/struct-derive-validate";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(target: &DerivedBoundaryTypeSite, results: &mut Vec<G3CheckResult>) {
    if target.boundary_kind != G3RsGardeBoundaryKind::Struct || target.has_validate {
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
fn check_input(input: &g3rs_garde_types::G3RsGardeSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for target in &input.struct_targets {
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
fn fixture(struct_targets: Vec<crate::support::DerivedBoundaryTypeSite>) -> Fixture {
    let mut input = crate::support::active_source_input();
    input.struct_targets = struct_targets;
    Fixture(input)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
