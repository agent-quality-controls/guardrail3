mod rule;
pub use rule::check;
#[cfg(test)]
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
pub(crate) use rule::DomainPurityEdgeKindForTest;
#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
#[cfg(test)]
use crate::dependency_facts::EdgeKind;
#[cfg(test)]
use crate::inputs::MemberDependencyHexarchInput;
#[cfg(test)]
pub(crate) fn run_domain_purity_case(
    tree: &ProjectTree,
    member_rel_dir: &str,
    edge_kind: DomainPurityEdgeKindForTest,
) -> Vec<CheckResult> {
    let facts = crate::collect_dependency_facts_from_tree_for_tests(tree);
    let member = facts
        .members
        .iter()
        .find(|member| member.rel_dir == member_rel_dir)
        .unwrap_or_else(|| panic!("missing domain member `{member_rel_dir}`"));
    let edges = facts
        .edges
        .iter()
        .filter(|edge| {
            edge.source_rel_dir == member.rel_dir
                && matches!(
                    (edge_kind, edge.kind),
                    (
                        DomainPurityEdgeKindForTest::Dependency,
                        EdgeKind::Dependency
                    ) | (
                        DomainPurityEdgeKindForTest::DevDependency,
                        EdgeKind::DevDependency
                    ) | (
                        DomainPurityEdgeKindForTest::BuildDependency,
                        EdgeKind::BuildDependency
                    ) | (
                        DomainPurityEdgeKindForTest::TargetDependency,
                        EdgeKind::TargetDependency
                    ) | (
                        DomainPurityEdgeKindForTest::TargetBuildDependency,
                        EdgeKind::TargetBuildDependency
                    )
                )
        })
        .collect();
    let mut results = Vec::new();
    check(
        &MemberDependencyHexarchInput::new(member, edges),
        &mut results,
    );
    results
}
#[cfg(test)]
pub(crate) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]

mod tests;
