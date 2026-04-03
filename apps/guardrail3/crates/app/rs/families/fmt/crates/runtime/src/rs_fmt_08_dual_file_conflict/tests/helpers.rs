use super::super::{check};
use guardrail3_domain_report::CheckResult;
use crate::inputs::RustfmtDualConflictInput;
pub(super) fn run_check(dir_rel: &str) -> Vec<CheckResult> {
    let input = RustfmtDualConflictInput {
        dir_rel: dir_rel.to_owned(),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
pub(super) fn run_family_check(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}
