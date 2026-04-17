use guardrail3_check_types::G3CheckResult;

use crate::support::{BoundaryFieldSite, error};

const ID: &str = "RS-GARDE-SOURCE-05";

pub(crate) fn check(field: &BoundaryFieldSite, results: &mut Vec<G3CheckResult>) {
    if !field.requires_field_validation
        || field.has_garde_skip
        || field.nested_validated
        || field.has_garde_dive
        || field.has_meaningful_garde_rule
    {
        return;
    }

    results.push(error(
        ID,
        format!(
            "boundary field `{}` missing garde validator",
            field.field_name
        ),
        format!(
            "Field `{}` in validated boundary `{}` has type `{}` but no meaningful garde validator. Add a field-level garde rule such as `length`, `range`, `url`, or another explicit validator.",
            field.field_name, field.boundary_name, field.field_type
        ),
        &field.rel_path,
        Some(field.line),
    ));
}

#[cfg(test)]
fn check_input(input: &g3rs_garde_types::G3RsGardeSourceChecksInput) -> Vec<G3CheckResult> {
    let analysis = crate::support::analyze_input(input);
    let mut results = Vec::new();
    for field in &analysis.boundary_fields {
        check(field, &mut results);
    }
    results
}

#[cfg(test)]
struct Fixture(crate::support::TestFixture);

#[cfg(test)]
impl Fixture {
    fn run(&self) -> Vec<G3CheckResult> {
        check_input(&self.0.input)
    }
}

#[cfg(test)]
fn fixture(source_files: &[(&str, &str)], rust_policy_toml: &str) -> Fixture {
    Fixture(crate::support::fixture(source_files, rust_policy_toml))
}

#[cfg(test)]
fn default_guardrail_toml() -> &'static str {
    crate::support::default_guardrail_toml()
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
