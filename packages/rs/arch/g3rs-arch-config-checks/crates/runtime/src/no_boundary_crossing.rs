use g3rs_arch_types::types::{G3RsArchBoundaryRef, G3RsArchDependencyEdge};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-arch/no-boundary-crossing";

fn is_allowed_test_edge(edge: &G3RsArchDependencyEdge) -> bool {
    (edge.source_rel_dir == "crates/assertions"
        && edge.resolved_target_rel.as_deref() == Some("crates/runtime"))
        || (edge.source_rel_dir == "crates/runtime"
            && edge.section == "dev-dependencies"
            && matches!(
                edge.resolved_target_rel.as_deref(),
                Some("crates/assertions" | "crates/test_support")
            ))
}

pub(crate) fn check(edge: &G3RsArchDependencyEdge, results: &mut Vec<G3CheckResult>) {
    let Some(target_rel) = &edge.resolved_target_rel else {
        return;
    };
    if !edge.target_is_crate {
        return;
    }
    if is_allowed_test_edge(edge) {
        return;
    }
    if edge.target_shared {
        return;
    }

    let Some(boundary) = &edge.crossed_boundary else {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "dependency does not cross crate boundary".to_owned(),
                format!(
                    "`{}` depends on `{}` without crossing a crate boundary.",
                    edge.source_rel_dir, target_rel
                ),
                Some(edge.source_cargo_rel.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    };

    let boundary_label = match boundary {
        G3RsArchBoundaryRef::RootWorkspace => "the root workspace".to_owned(),
        G3RsArchBoundaryRef::Crate(rel_dir) => format!("`{rel_dir}`"),
    };

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "dependency crosses crate boundary".to_owned(),
        format!(
            "`{}` depends on `{}` (via `{}`), but this crosses the boundary of {boundary_label}. Depending on internal crates couples you to another package's structure. Depend on {boundary_label}'s facade crate instead.",
            edge.source_rel_dir, target_rel, edge.dep_alias
        ),
        Some(edge.source_cargo_rel.clone()),
        None,
    ));
}
