use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ExceptionCommentInput;

const ID: &str = "RS-CODE-07";

pub fn check(input: &ExceptionCommentInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "EXCEPTION comment inventory".to_owned(),
            message: format!("Config exception comment: {}", input.line_text),
            file: Some(input.rel_path.to_owned()),
            line: Some(input.line),
            inventory: false,
        }
        .as_inventory(),
    );
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) fn copy_fixture() -> test_support::TempDir {
    crate::copy_test_fixture()
}

#[cfg(test)]
pub(crate) fn check_comment(rel_path: &str, line: usize, line_text: &str) -> Vec<CheckResult> {
    let input = super::inputs::ExceptionCommentInput {
        rel_path,
        line,
        line_text,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_07_exception_comment_inventory_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_code_07_exception_comment_inventory_tests;
