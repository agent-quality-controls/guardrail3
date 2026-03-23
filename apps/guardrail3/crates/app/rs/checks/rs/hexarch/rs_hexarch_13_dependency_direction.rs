use crate::domain::report::{CheckResult, Severity};

use super::inputs::DependencyEdgeHexarchInput;

const ID: &str = "RS-HEXARCH-13";

pub fn check(input: &DependencyEdgeHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let edge = input.edge;
    if edge.kind.is_dev() || edge.kind.is_target() || edge.is_workspace_inherited {
        return;
    }
    if edge.dep_alias != edge.dep_package_name {
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
        title: "dependency direction violation".to_owned(),
        message: format!(
            "{} crate `{}` ({}) depends on {} crate `{}` via `{}`.",
            source_layer.label(),
            edge.source_name,
            edge.source_rel_dir,
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
#[path = "rs_hexarch_13_dependency_direction_tests/mod.rs"]
mod tests;
