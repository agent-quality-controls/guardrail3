mod rule;
pub use rule::{check};

#[cfg(test)]
pub(crate) fn run_family_check(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}
#[cfg(test)]

mod tests;
