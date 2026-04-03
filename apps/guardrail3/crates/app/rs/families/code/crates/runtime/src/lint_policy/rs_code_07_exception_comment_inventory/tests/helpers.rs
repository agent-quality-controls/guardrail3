use super::super::{check};
use guardrail3_domain_report::CheckResult;
pub(super) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}
pub(super) fn copy_fixture() -> test_support::TempDir {
    crate::copy_test_fixture()
}
pub(super) fn check_comment(rel_path: &str, line: usize, line_text: &str) -> Vec<CheckResult> {
    let input = crate::inputs::ExceptionCommentInput {
        rel_path,
        line,
        line_text,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
