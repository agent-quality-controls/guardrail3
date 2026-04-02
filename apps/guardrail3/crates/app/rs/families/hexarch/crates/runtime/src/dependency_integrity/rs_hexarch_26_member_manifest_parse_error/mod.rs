mod rule;
pub use rule::{check, check_inventory};
#[cfg(test)]
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
pub(crate) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]

mod tests;
