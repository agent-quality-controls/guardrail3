mod rule;
pub use rule::{check};

#[cfg(test)]
pub(crate) fn run_check(config_rel: &str, config_kind: RustfmtConfigKind) -> Vec<CheckResult> {
    let input = RustfmtExtraConfigInput {
        config_rel: config_rel.to_owned(),
        config_kind,
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
