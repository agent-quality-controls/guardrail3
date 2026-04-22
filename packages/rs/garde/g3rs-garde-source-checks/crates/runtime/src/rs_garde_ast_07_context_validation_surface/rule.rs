use guardrail3_check_types::G3CheckResult;

use crate::support::{BoundaryFieldSite, error};

const ID: &str = "RS-GARDE-SOURCE-07";

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

#[cfg(test)]
fn check_input(input: &g3rs_garde_types::G3RsGardeSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for field in &input.boundary_fields {
        check(field, &mut results);
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
fn fixture(boundary_fields: Vec<crate::support::BoundaryFieldSite>) -> Fixture {
    let mut input = crate::support::active_source_input();
    input.boundary_fields = boundary_fields;
    Fixture(input)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
