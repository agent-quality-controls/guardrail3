mod rule;
pub use rule::check;
#[cfg(test)]
use guardrail3_domain_report::CheckResult;
pub(crate) use rule::has_owned_assertion_proof;

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    crate::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]

mod tests;
