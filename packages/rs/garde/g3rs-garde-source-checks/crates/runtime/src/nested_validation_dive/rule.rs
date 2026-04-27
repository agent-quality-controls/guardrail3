use guardrail3_check_types::G3CheckResult;

use crate::support::{BoundaryFieldSite, error};

const ID: &str = "g3rs-garde/nested-validation-dive";

pub(crate) fn check(field: &BoundaryFieldSite, results: &mut Vec<G3CheckResult>) {
    if !field.nested_validated || field.has_garde_skip || field.has_garde_dive {
        return;
    }

    results.push(error(
        ID,
        format!(
            "nested validated field `{}` missing garde(dive)",
            field.field_name
        ),
        format!(
            "Field `{}` in validated boundary `{}` points at validated nested type `{}` but is missing `#[garde(dive)]`. Nested validated fields must opt into recursive garde validation.",
            field.field_name, field.boundary_name, field.field_type
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
