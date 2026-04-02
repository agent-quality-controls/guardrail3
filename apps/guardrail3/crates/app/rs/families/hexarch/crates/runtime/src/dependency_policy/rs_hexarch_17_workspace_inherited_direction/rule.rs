use guardrail3_domain_report::{CheckResult, Severity};


use crate::inputs::DependencyEdgeHexarchInput;
use crate::inventory::push_success;

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
        push_success(
            results,
            ID,
            "workspace dependency direction allowed".to_owned(),
            format!(
                "{} crate `{}` inherits workspace dependency `{}` to {} crate `{}` without violating hexarch direction.",
                source_layer.label(),
                edge.source_name,
                edge.dep_alias,
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
        "workspace dependency direction violation".to_owned(),
        format!(
            "{} crate `{}` inherits workspace dependency `{}` which resolves to {} crate `{}`.",
            source_layer.label(),
            edge.source_name,
            edge.dep_alias,
            target_layer.label(),
            edge.dep_package_name
        ),
        Some(edge.source_cargo_rel_path.clone()),
        None,
        false,
    ));
}


#[cfg(test)]
#[derive(Debug)]
pub struct WorkspaceInheritedDirectionAudit {
    pub rule17: Vec<CheckResult>,
    pub rule18: Vec<CheckResult>,
    pub rule24: Vec<CheckResult>,
}

