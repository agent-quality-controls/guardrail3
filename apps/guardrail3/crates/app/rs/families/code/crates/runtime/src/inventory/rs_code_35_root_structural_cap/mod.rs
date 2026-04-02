mod rule;
pub use rule::{check};
#[cfg(test)]
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}
#[cfg(test)]

mod tests;
