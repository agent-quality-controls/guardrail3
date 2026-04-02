mod rule;
pub use rule::check;
#[cfg(test)]
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
pub(crate) use rule::WorkspaceInheritedDirectionAudit;
#[cfg(test)]
use crate::inputs::DependencyEdgeHexarchInput;
#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
#[cfg(test)]
pub fn results_for_dependency_edges_for_test(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = crate::collect_dependency_facts_from_tree_for_tests(tree);
    let mut results = Vec::new();
    for edge in facts
        .edges
        .iter()
        .filter(|edge| edge.kind == crate::dependency_facts::EdgeKind::Dependency)
    {
        check(&DependencyEdgeHexarchInput::new(edge), &mut results);
    }
    results
}
#[cfg(test)]
pub fn audit_edge_for_test(
    tree: &ProjectTree,
    source_rel_dir: &str,
) -> WorkspaceInheritedDirectionAudit {
    let facts = crate::collect_dependency_facts_from_tree_for_tests(tree);
    let edge = facts
        .edges
        .iter()
        .find(|edge| edge.source_rel_dir == source_rel_dir)
        .expect("expected inherited dependency edge from workspace member");

    let mut rule17 = Vec::new();
    check(&DependencyEdgeHexarchInput::new(edge), &mut rule17);

    let mut rule18 = Vec::new();
    crate::dependency_policy::rs_hexarch_18_renamed_dependency_direction::check(
        &DependencyEdgeHexarchInput::new(edge),
        &mut rule18,
    );

    let mut rule24 = Vec::new();
    crate::dependency_integrity::rs_hexarch_24_cross_app_boundary::check(
        &DependencyEdgeHexarchInput::new(edge),
        &mut rule24,
    );

    WorkspaceInheritedDirectionAudit {
        rule17,
        rule18,
        rule24,
    }
}
#[cfg(test)]
pub(crate) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]

mod tests;
