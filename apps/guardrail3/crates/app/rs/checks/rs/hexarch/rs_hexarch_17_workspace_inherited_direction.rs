use crate::domain::report::{CheckResult, Severity};

use super::inputs::DependencyEdgeHexarchInput;

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
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "workspace dependency direction violation".to_owned(),
        message: format!(
            "{} crate `{}` inherits workspace dependency `{}` which resolves to {} crate `{}`.",
            source_layer.label(),
            edge.source_name,
            edge.dep_alias,
            target_layer.label(),
            edge.dep_package_name
        ),
        file: Some(edge.source_cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_hexarch_17_workspace_inherited_direction_tests/mod.rs"]
mod tests;
