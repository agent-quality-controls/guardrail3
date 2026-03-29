use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::CodeInputFailureInput;

const ID: &str = "RS-CODE-30";

pub fn check(input: &CodeInputFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "code-family input failure".to_owned(),
        message: input.message.to_owned(),
        file: Some(input.rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
pub(crate) fn run_tree(tree: &guardrail3_domain_project_tree::ProjectTree) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
pub(crate) use guardrail3_domain_project_tree::{DirEntry, ProjectTree};

#[cfg(test)]
pub(crate) fn check_input_failure(rel_path: &str, message: &str) -> Vec<CheckResult> {
    let input = super::inputs::CodeInputFailureInput { rel_path, message };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_30_input_failures_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_code_30_input_failures_tests;
