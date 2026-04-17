use guardrail3_check_types::G3CheckResult;

use crate::support::{InputFailureSite, error};

const ID: &str = "RS-GARDE-SOURCE-10";

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
    let analysis = crate::support::analyze_input(input);
    let mut results = Vec::new();
    for failure in &analysis.input_failures {
        check(failure, &mut results);
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

    #[cfg(unix)]
    fn make_source_unreadable(&self, rel_path: &str) {
        self.0.make_source_unreadable(rel_path);
    }
}

#[cfg(test)]
fn fixture(source_files: &[(&str, &str)], rust_policy_toml: &str) -> Fixture {
    Fixture(crate::support::fixture(source_files, rust_policy_toml))
}

#[cfg(test)]
fn invalid_policy_fixture(source_files: &[(&str, &str)], message: &str) -> Fixture {
    Fixture(crate::support::invalid_policy_fixture(source_files, message))
}

#[cfg(test)]
fn default_guardrail_toml() -> &'static str {
    crate::support::default_guardrail_toml()
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
