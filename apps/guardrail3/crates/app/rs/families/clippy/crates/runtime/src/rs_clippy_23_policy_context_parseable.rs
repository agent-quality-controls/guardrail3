#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PolicyContextFailureInput;

const ID: &str = "RS-CLIPPY-23";

pub fn check(input: &PolicyContextFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "clippy policy context is not parseable".to_owned(),
        message: format!(
            "Failed to parse active `guardrail3.toml` used for clippy profile and garde policy: {}",
            input.parse_error
        ),
        file: Some("guardrail3.toml".to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
pub(crate) fn run_for_tests(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = super::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    if let Some(parse_error) = facts.policy_context_parse_error.as_deref() {
        check(&PolicyContextFailureInput::new(parse_error), &mut results);
    }
    results
}

#[cfg(test)]
#[path = "rs_clippy_23_policy_context_parseable_tests/mod.rs"]
mod rs_clippy_23_policy_context_parseable_tests;
