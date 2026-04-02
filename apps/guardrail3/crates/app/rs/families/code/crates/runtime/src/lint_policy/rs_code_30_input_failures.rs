use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::CodeInputFailureInput;

const ID: &str = "RS-CODE-30";

pub fn check(input: &CodeInputFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "code-family input failure".to_owned(),
        input.message.to_owned(),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

#[cfg(test)]
pub(crate) fn run_tree(tree: &guardrail3_app_rs_family_view::FamilyView) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
pub(crate) use guardrail3_app_rs_family_view::{DirEntry, FamilyView as ProjectTree};

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
