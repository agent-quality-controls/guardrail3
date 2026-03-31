use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::DependencyEdgeHexarchInput;
use super::inventory::push_success;

const ID: &str = "RS-HEXARCH-24";

pub fn check(input: &DependencyEdgeHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let edge = input.edge;
    let (Some(source_app), Some(target_app)) = (
        edge.source_app_root_rel_dir.as_deref(),
        edge.target_app_root_rel_dir.as_deref(),
    ) else {
        return;
    };
    if source_app == target_app
        || edge.resolved_target_rel_dir.is_none()
        || !edge.resolved_target_exists
    {
        if source_app == target_app
            && edge.resolved_target_rel_dir.is_some()
            && edge.resolved_target_exists
        {
            push_success(
                results,
                ID,
                "dependency stays within one app boundary".to_owned(),
                format!(
                    "`{}` depends on `{}` without crossing app boundaries.",
                    edge.source_rel_dir, edge.dep_package_name
                ),
                Some(edge.source_cargo_rel_path.clone()),
            );
        }
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    "cross-app boundary dependency".to_owned(),
    format!(
            "`{}` depends on `{}` across app boundaries ({} -> {}). Cross-app sharing must go through `packages/` or external APIs.",
            edge.source_rel_dir, edge.dep_package_name, source_app, target_app
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
    tree: &guardrail3_app_rs_family_mapper::RsProjectSurface,
) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_hexarch_24_cross_app_boundary_tests/mod.rs"]
mod rs_hexarch_24_cross_app_boundary_tests;
