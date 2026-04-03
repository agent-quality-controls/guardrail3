pub use super::super::check;

pub(super) fn run_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree(tree, tc, thorough)
}
pub(super) fn run_tree_with_validation_scope(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
    validation_scope: &str,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree_with_validation_scope(tree, tc, thorough, validation_scope)
}
pub(super) fn crate_facts(name: &str) -> crate::facts::PublishableCrateFacts {
    crate::test_fixtures::crate_facts(name)
}
pub(super) fn crate_input(
    krate: &crate::facts::PublishableCrateFacts,
) -> crate::inputs::PublishableCrateReleaseInput<'_> {
    crate::test_fixtures::crate_input(krate)
}
pub(super) use test_support::{StubToolChecker, dir_entry, project_tree, temp_root};
