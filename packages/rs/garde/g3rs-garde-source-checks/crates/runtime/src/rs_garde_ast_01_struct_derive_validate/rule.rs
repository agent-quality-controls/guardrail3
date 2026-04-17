use guardrail3_check_types::G3CheckResult;

use crate::parse::BoundaryKind;
use crate::support::{DerivedBoundaryTypeSite, error};

const ID: &str = "RS-GARDE-SOURCE-01";

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
fn check_input(input: &g3rs_garde_types::G3RsGardeSourceChecksInput) -> Vec<G3CheckResult> {
    let analysis = crate::support::analyze_input(input);
    let mut results = Vec::new();
    for target in &analysis.struct_targets {
        check(target, &mut results);
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
