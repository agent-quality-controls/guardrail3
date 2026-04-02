mod rule;
pub use rule::{check};
#[cfg(test)]
use guardrail3_domain_report::CheckResult;

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
    let input = crate::inputs::ExceptionCommentInput {
        rel_path,
        line,
        line_text,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
mod tests;
