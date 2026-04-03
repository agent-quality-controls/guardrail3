use guardrail3_domain_report::CheckResult;
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
pub(super) fn results_for_test_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}
