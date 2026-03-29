use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::DependencyEdgeHexarchInput;

const ID: &str = "RS-HEXARCH-14";

pub fn check(input: &DependencyEdgeHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let edge = input.edge;
    if !edge.resolved_target_exists {
        return;
    }
    let Some(target) = &edge.resolved_target_rel_dir else {
        return;
    };

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Info,
        title: format!(
            "hexarch path dependency `{}` -> `{}`",
            edge.source_name, edge.dep_alias
        ),
        message: format!(
            "`{}` depends on `{}` via `{}` resolved to `{}`.",
            edge.source_rel_dir, edge.dep_package_name, edge.section_label, target
        ),
        file: Some(edge.source_cargo_rel_path.clone()),
        line: None,
        inventory: true,
    });
}

#[cfg(test)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]
#[path = "rs_hexarch_14_dependency_inventory_tests/mod.rs"]
mod rs_hexarch_14_dependency_inventory_tests;
