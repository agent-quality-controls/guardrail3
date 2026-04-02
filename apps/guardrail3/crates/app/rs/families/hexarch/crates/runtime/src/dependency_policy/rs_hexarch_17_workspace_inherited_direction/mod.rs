use guardrail3_domain_report::{CheckResult, Severity};

#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use crate::inputs::DependencyEdgeHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-17";

pub fn check(input: &DependencyEdgeHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let edge = input.edge;
    if !edge.is_workspace_inherited || edge.kind.is_dev() || edge.kind.is_target() {
        return;
    }
    if edge.source_app_root_rel_dir != edge.target_app_root_rel_dir
        && edge.source_app_root_rel_dir.is_some()
        && edge.target_app_root_rel_dir.is_some()
    {
        return;
    }
    let (Some(source_layer), Some(target_layer)) = (edge.source_layer, edge.target_layer) else {
        return;
    };
    if !source_layer.forbidden().contains(&target_layer) {
        push_success(
            results,
            ID,
            "workspace dependency direction allowed".to_owned(),
            format!(
                "{} crate `{}` inherits workspace dependency `{}` to {} crate `{}` without violating hexarch direction.",
                source_layer.label(),
                edge.source_name,
                edge.dep_alias,
                target_layer.label(),
                edge.dep_package_name
            ),
            Some(edge.source_cargo_rel_path.clone()),
        );
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "workspace dependency direction violation".to_owned(),
        format!(
            "{} crate `{}` inherits workspace dependency `{}` which resolves to {} crate `{}`.",
            source_layer.label(),
            edge.source_name,
            edge.dep_alias,
            target_layer.label(),
            edge.dep_package_name
        ),
        Some(edge.source_cargo_rel_path.clone()),
        None,
        false,
    ));
}

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
#[derive(Debug)]
pub struct WorkspaceInheritedDirectionAudit {
    pub rule17: Vec<CheckResult>,
    pub rule18: Vec<CheckResult>,
    pub rule24: Vec<CheckResult>,
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
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]

mod rs_hexarch_17_workspace_inherited_direction_tests;
