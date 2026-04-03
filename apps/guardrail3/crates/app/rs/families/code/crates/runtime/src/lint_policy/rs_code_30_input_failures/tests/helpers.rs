use super::super::{check};
use guardrail3_domain_report::CheckResult;
pub(super) fn run_tree(tree: &guardrail3_app_rs_family_view::FamilyView) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}
pub(super) use guardrail3_app_rs_family_view::{DirEntry, FamilyView as ProjectTree};
pub(super) fn check_input_failure(rel_path: &str, message: &str) -> Vec<CheckResult> {
    let input = crate::inputs::CodeInputFailureInput { rel_path, message };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
