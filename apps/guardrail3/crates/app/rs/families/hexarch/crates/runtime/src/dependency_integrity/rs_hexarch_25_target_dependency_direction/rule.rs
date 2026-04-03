use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::DependencyEdgeHexarchInput;
use crate::inventory::push_success;

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
            "{} crate `{}` has target-specific dependency on {} crate `{}`. {} must not depend on {}. Remove this dependency or invert the direction through ports.",
            source_layer.label(),
            edge.source_name,
            target_layer.label(),
            edge.dep_package_name,
            source_layer.label(),
            target_layer.label()
        ),
        Some(edge.source_cargo_rel_path.clone()),
        None,
        false,
    ));
}

