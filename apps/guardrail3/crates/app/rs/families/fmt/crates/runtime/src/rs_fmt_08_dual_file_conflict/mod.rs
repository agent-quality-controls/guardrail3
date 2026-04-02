mod rule;
pub use rule::{check};

#[cfg(test)]
pub(crate) fn run_check(dir_rel: &str) -> Vec<CheckResult> {
    let input = RustfmtDualConflictInput {
        dir_rel: dir_rel.to_owned(),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
#[cfg(test)]
pub(crate) fn run_family_check(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}
#[cfg(test)]

mod tests;
