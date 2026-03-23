use crate::domain::report::{CheckResult, Severity};

use super::inputs::DependencyEdgeHexarchInput;

const ID: &str = "RS-HEXARCH-18";

pub fn check(input: &DependencyEdgeHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let edge = input.edge;
    if edge.dep_alias == edge.dep_package_name || edge.kind.is_dev() || edge.kind.is_target() {
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
        title: "renamed dependency direction violation".to_owned(),
        message: format!(
            "{} crate `{}` depends on alias `{}` for package `{}` which resolves to {} layer `{}`.",
            source_layer.label(),
            edge.source_name,
            edge.dep_alias,
            edge.dep_package_name,
            target_layer.label(),
            edge.resolved_target_rel_dir
                .as_deref()
                .unwrap_or("<unknown>")
        ),
        file: Some(edge.source_cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_hexarch_18_renamed_dependency_direction_tests/mod.rs"]
mod tests;
