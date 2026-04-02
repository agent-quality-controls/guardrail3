use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::DependencyEdgeHexarchInput;
use super::inventory::push_success;

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
        push_success(
            results,
            ID,
            "target dependency direction allowed".to_owned(),
            format!(
                "{} crate `{}` has target-specific dependency on {} crate `{}` without violating hexarch direction.",
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
        Severity::Error,
        "target dependency direction violation".to_owned(),
        format!(
            "{} crate `{}` has target-specific dependency on {} crate `{}`.",
            source_layer.label(),
            edge.source_name,
            target_layer.label(),
            edge.dep_package_name
        ),
        Some(edge.source_cargo_rel_path.clone()),
        None,
        false,
    ));
}

#[cfg(test)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}

#[cfg(test)]
pub(super) fn results_for_test_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_hexarch_25_target_dependency_direction_tests/mod.rs"]
mod rs_hexarch_25_target_dependency_direction_tests;
