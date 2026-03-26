use guardrail3_domain_report::{CheckResult, Severity};

use super::dependency_facts::EdgeKind;
use super::inputs::DependencyEdgeHexarchInput;

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
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "dev-dependency direction violation".to_owned(),
        message: format!(
            "{} crate `{}` dev-depends on {} crate `{}` via `{}`.",
            source_layer.label(),
            edge.source_name,
            target_layer.label(),
            edge.dep_package_name,
            edge.section_label
        ),
        file: Some(edge.source_cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_hexarch_20_dev_dependency_direction_tests/mod.rs"]
mod rs_hexarch_20_dev_dependency_direction_tests;
