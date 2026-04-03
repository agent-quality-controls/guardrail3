use guardrail3_domain_report::CheckResult;
pub(super) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}
pub(super) fn copy_fixture() -> test_support::TempDir {
    crate::copy_test_fixture()
}
