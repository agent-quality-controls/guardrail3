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
pub(crate) fn check_unsafe_code_lint(
    cargo_rel_path: &str,
    lint_level: Option<&str>,
) -> Vec<CheckResult> {
    let input = crate::inputs::UnsafeCodeLintInput {
        cargo_rel_path,
        lint_level,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
#[cfg(test)]

mod tests;
