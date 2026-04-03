use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::{CrateTree, DependencyEdge};

const ID: &str = "RS-ARCH-05";

pub(crate) fn check(
    edge: &DependencyEdge,
    crate_tree: &CrateTree,
    results: &mut Vec<CheckResult>,
) {
    let Some(target_rel) = &edge.resolved_target_rel else {
        return;
    };
    if !edge.target_is_crate {
        return;
    }

    // Check if the dependency crosses a crate boundary that the source is not inside of.
    let violation = crate_tree.boundary_violation(&edge.source_rel_dir, target_rel);

    let Some(boundary_rel) = violation else {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "dependency does not cross crate boundary".to_owned(),
                format!(
                    "`{}` depends on `{}` without crossing a crate boundary.",
                    edge.source_rel_dir, target_rel
                ),
                Some(edge.source_cargo_rel.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    };

    let boundary_label = if boundary_rel.is_empty() {
        "the root workspace".to_owned()
    } else {
        format!("`{boundary_rel}`")
    };

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "dependency crosses crate boundary".to_owned(),
        format!(
            "`{}` depends on `{}` (via `{}`), but this crosses the boundary of {boundary_label}. Depending on internal crates couples you to another package's structure. Depend on {boundary_label}'s facade crate instead.",
            edge.source_rel_dir,
            target_rel,
            edge.dep_alias,
        ),
        Some(edge.source_cargo_rel.clone()),
        None,
        false,
    ));
}
