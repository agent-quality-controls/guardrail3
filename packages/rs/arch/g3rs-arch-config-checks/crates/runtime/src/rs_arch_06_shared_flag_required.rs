use g3rs_arch_types::G3RsArchConfigCrate;
use g3rs_arch_types::G3RsArchDependencyEdge;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use std::collections::BTreeMap;

const ID: &str = "RS-ARCH-CONFIG-06";

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

pub(crate) fn check(
    edge: &G3RsArchDependencyEdge,
    crate_map: &BTreeMap<&str, &G3RsArchConfigCrate>,
    results: &mut Vec<G3CheckResult>,
) {
    let Some(target_rel) = &edge.resolved_target_rel else {
        return;
    };
    if !edge.target_is_crate {
        return;
    }
    if is_allowed_test_edge(edge) {
        return;
    }
    let Some(_target_node) = crate_map.get(target_rel.as_str()) else {
        return;
    };

    if edge.is_direct_child {
        return;
    }

    if edge.target_shared {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "non-child dependency target is shared".to_owned(),
                format!(
                    "`{}` depends on `{}` which is marked `shared = true`.",
                    edge.source_rel_dir, target_rel
                ),
                Some(edge.source_cargo_rel.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "non-child dependency requires shared flag".to_owned(),
        format!(
            "`{}` depends on `{}` (via `{}`), but `{}` is not marked as shared. Dependencies on non-child crates are forbidden by default to prevent coupling. If this crate is genuinely a shared dependency (types, contracts, utilities), add `shared = true` under `[package.metadata.guardrail3]` in its Cargo.toml. Otherwise, depend on the parent facade instead.",
            edge.source_rel_dir, target_rel, edge.dep_alias, target_rel
        ),
        Some(edge.source_cargo_rel.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "rs_arch_06_shared_flag_required_tests/mod.rs"]
// reason: keep rule tests in the owned x_tests sidecar directory.
mod rs_arch_06_shared_flag_required_tests;
