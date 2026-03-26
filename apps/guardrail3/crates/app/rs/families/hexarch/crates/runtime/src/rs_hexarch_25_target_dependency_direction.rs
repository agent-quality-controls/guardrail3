use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::DependencyEdgeHexarchInput;

const ID: &str = "RS-HEXARCH-25";

pub fn check(input: &DependencyEdgeHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let edge = input.edge;
    if !edge.kind.is_target() || !edge.resolved_target_exists {
        return;
    }
    if edge.source_app_root_rel_dir.is_some()
        && edge.target_app_root_rel_dir.is_some()
        && edge.source_app_root_rel_dir != edge.target_app_root_rel_dir
    {
        return;
    }
    let (Some(source_layer), Some(target_layer)) = (edge.source_layer, edge.target_layer) else {
        return;
    };
    if !source_layer.forbidden().contains(&target_layer) {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "target dependency direction violation".to_owned(),
        message: format!(
            "{} crate `{}` has target-specific dependency on {} crate `{}`.",
            source_layer.label(),
            edge.source_name,
            target_layer.label(),
            edge.dep_package_name
        ),
        file: Some(edge.source_cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[allow(dead_code)]
pub fn check_tree_for_tests(tree: &guardrail3_domain_project_tree::ProjectTree) -> Vec<CheckResult> {
    let dependency_facts = super::collect_dependency_facts_for_tests(tree, &super::family_route_for_tests(tree));
    let mut results = Vec::new();
    for edge in &dependency_facts.edges {
        check(&DependencyEdgeHexarchInput::new(edge), &mut results);
    }
    results
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_tree(tree: &guardrail3_domain_project_tree::ProjectTree) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_hexarch_25_target_dependency_direction_tests/mod.rs"]
mod rs_hexarch_25_target_dependency_direction_tests;
