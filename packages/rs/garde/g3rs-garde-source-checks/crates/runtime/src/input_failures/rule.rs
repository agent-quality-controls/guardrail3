use guardrail3_check_types::G3CheckResult;

use crate::support::{InputFailureSite, error};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-garde/input-failures";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(site: &InputFailureSite, results: &mut Vec<G3CheckResult>) {
    results.push(error(
        ID,
        "garde-family input failure",
        site.message.clone(),
        site.rel_path.as_str(),
        None,
    ));
}

#[cfg(test)]
fn check_input(input: &g3rs_garde_types::G3RsGardeSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for failure in &input.input_failures {
        check(failure, &mut results);
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
fn fixture(input_failures: Vec<crate::support::InputFailureSite>) -> Fixture {
    let mut input = crate::support::active_source_input();
    input.input_failures = input_failures;
    Fixture(input)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
