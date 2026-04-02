use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::DependencyEdgeHexarchInput;

const ID: &str = "RS-HEXARCH-14";

pub fn check(input: &DependencyEdgeHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let edge = input.edge;
    if !edge.resolved_target_exists {
        return;
    }
    let Some(target) = &edge.resolved_target_rel_dir else {
        return;
    };

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Info,
        format!(
            "hexarch path dependency `{}` -> `{}`",
            edge.source_name, edge.dep_alias
        ),
        format!(
            "`{}` depends on `{}` via `{}` resolved to `{}`.",
            edge.source_rel_dir, edge.dep_package_name, edge.section_label, target
        ),
        Some(edge.source_cargo_rel_path.clone()),
        None,
        true,
    ));
}

