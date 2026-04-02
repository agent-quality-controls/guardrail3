mod rule;
pub use rule::{check};

#[cfg(test)]
pub(crate) fn run_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree(tree, tc, thorough)
}
#[cfg(test)]
pub(crate) fn run_tree_with_validation_scope(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
    validation_scope: &str,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree_with_validation_scope(tree, tc, thorough, validation_scope)
}
#[cfg(test)]
pub(crate) fn repo_facts() -> crate::facts::RepoReleaseFacts {
    crate::test_fixtures::repo_facts()
}
#[cfg(test)]
pub(crate) fn repo_input(
    repo: &crate::facts::RepoReleaseFacts,
) -> crate::inputs::RepoReleaseInput<'_> {
    crate::test_fixtures::repo_input(repo)
}
#[cfg(test)]
pub(super) use test_support::{StubToolChecker, dir_entry, project_tree, temp_root};
#[cfg(test)]

mod tests;
