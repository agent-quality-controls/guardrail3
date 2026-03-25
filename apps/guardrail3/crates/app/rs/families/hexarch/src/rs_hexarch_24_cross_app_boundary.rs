use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::DependencyEdgeHexarchInput;

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
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "cross-app boundary dependency".to_owned(),
        message: format!(
            "`{}` depends on `{}` across app boundaries ({} -> {}). Cross-app sharing must go through `packages/` or external APIs.",
            edge.source_rel_dir, edge.dep_package_name, source_app, target_app
        ),
        file: Some(edge.source_cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_hexarch_24_cross_app_boundary_tests/mod.rs"]
mod tests;
