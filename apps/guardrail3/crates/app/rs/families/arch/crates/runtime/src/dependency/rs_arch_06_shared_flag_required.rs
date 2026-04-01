use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::{CrateTree, DependencyEdge};

const ID: &str = "RS-ARCH-06";

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

    let Some(target_node) = crate_tree.nodes.get(target_rel.as_str()) else {
        return;
    };

    // If target is a direct child of source, no shared flag needed.
    if crate_tree.is_direct_child(&edge.source_rel_dir, target_rel) {
        return;
    }

    if target_node.shared {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "non-child dependency target is shared".to_owned(),
                format!(
                    "`{}` depends on `{}` which is marked `shared = true`.",
                    edge.source_rel_dir, target_rel
                ),
                Some(edge.source_cargo_rel.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "non-child dependency requires shared flag".to_owned(),
        format!(
            "`{}` depends on `{}` (via `{}`), but `{}` is not marked as shared. \
             Dependencies on non-child crates are forbidden by default to prevent coupling. \
             If this crate is genuinely a shared dependency (types, contracts, utilities), \
             add `shared = true` under `[package.metadata.guardrail3]` in its Cargo.toml. \
             Shared crates should ideally be named to reflect their shared nature \
             (e.g., shared, common, contracts, types). \
             Otherwise, depend on the parent facade instead.",
            edge.source_rel_dir,
            target_rel,
            edge.dep_alias,
            target_rel,
        ),
        Some(edge.source_cargo_rel.clone()),
        None,
        false,
    ));
}
