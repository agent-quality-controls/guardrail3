use guardrail3_domain_report::{CheckResult, Severity};

use super::dependency_facts::EdgeKind;
use super::inputs::DependencyEdgeHexarchInput;
use super::inventory::push_success;

const ID: &str = "RS-HEXARCH-20";

pub fn check(input: &DependencyEdgeHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let edge = input.edge;
    if edge.kind != EdgeKind::DevDependency || !edge.resolved_target_exists {
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
        push_success(
            results,
            ID,
            "dev-dependency direction allowed".to_owned(),
            format!(
                "{} crate `{}` dev-depends on {} crate `{}` without violating hexarch direction.",
                source_layer.label(),
                edge.source_name,
                target_layer.label(),
                edge.dep_package_name
            ),
            Some(edge.source_cargo_rel_path.clone()),
        );
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Warn,
        "dev-dependency direction violation".to_owned(),
        format!(
            "{} crate `{}` dev-depends on {} crate `{}` via `{}`.",
            source_layer.label(),
            edge.source_name,
            target_layer.label(),
            edge.dep_package_name,
            edge.section_label
        ),
        Some(edge.source_cargo_rel_path.clone()),
        None,
        false,
    ));
}

#[cfg(test)]
pub(crate) fn check_for_test_tree(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<CheckResult> {
    let route = super::family_route_for_tests(tree);
    super::check(tree, &route)
}

#[cfg(test)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]
#[path = "rs_hexarch_20_dev_dependency_direction_tests/mod.rs"]
mod rs_hexarch_20_dev_dependency_direction_tests;
