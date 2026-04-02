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
pub(crate) fn crate_facts(name: &str) -> crate::facts::PublishableCrateFacts {
    crate::test_fixtures::crate_facts(name)
}
#[cfg(test)]
pub(crate) fn crate_input(
    krate: &crate::facts::PublishableCrateFacts,
) -> crate::inputs::PublishableCrateReleaseInput<'_> {
    crate::test_fixtures::crate_input(krate)
}
#[cfg(test)]
pub(crate) fn repo_facts() -> crate::facts::RepoReleaseFacts {
    crate::test_fixtures::repo_facts()
}
#[cfg(test)]
pub(crate) fn workflow_from_yaml(rel_path: &str, yaml: &str) -> crate::facts::WorkflowFacts {
    crate::test_fixtures::workflow_from_yaml(rel_path, yaml)
}
#[cfg(test)]
pub(super) use test_support::{StubToolChecker, dir_entry, project_tree, temp_root};
#[cfg(test)]

mod tests;
