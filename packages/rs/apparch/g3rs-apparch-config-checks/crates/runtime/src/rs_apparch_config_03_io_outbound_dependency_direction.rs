use std::collections::BTreeMap;

use g3rs_apparch_types::{G3RsApparchCrate, G3RsApparchDependencyEdge, G3RsApparchLayer};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-APPARCH-CONFIG-03";

pub(crate) fn check(
    krate: &G3RsApparchCrate,
    crates_by_path: &BTreeMap<String, &G3RsApparchCrate>,
    dependency_edges: &[G3RsApparchDependencyEdge],
    results: &mut Vec<G3CheckResult>,
) {
    if krate.layer != Some(G3RsApparchLayer::IoOutbound) {
        return;
    }

    let violating = dependency_edges
        .iter()
        .filter(|edge| edge.from_cargo_rel_path == krate.cargo_rel_path)
        .filter(|edge| !edge.kind.is_dev())
        .filter_map(|edge| crates_by_path.get(&edge.to_cargo_rel_path).copied())
        .filter(|target| {
            matches!(
                target.layer,
                Some(G3RsApparchLayer::Logic)
                    | Some(G3RsApparchLayer::IoInbound)
                    | Some(G3RsApparchLayer::IoOutbound)
            ) && !crate::run::is_package_internal_assertions_to_runtime_edge(krate, target)
        })
        .collect::<Vec<_>>();

    if violating.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                format!(
                    "io/outbound crate `{}` depends only on allowed layers",
                    crate::run::display_crate(krate)
                ),
                format!(
                    "io/outbound crate `{}` has no workspace-internal dependencies on forbidden apparch layers.",
                    crate::run::display_crate(krate)
                ),
                Some(krate.cargo_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    for target in violating {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            format!(
                "io/outbound crate `{}` depends on forbidden crate `{}`",
                crate::run::display_crate(krate),
                crate::run::display_crate(target)
            ),
            format!(
                "io/outbound crate `{}` must not depend on other `io/outbound/` crates, `logic/`, or `io/inbound/`. Remove the dependency on `{}` or extract a shared contract into `types/`.",
                crate::run::display_crate(krate),
                crate::run::display_crate(target)
            ),
            Some(krate.cargo_rel_path.clone()),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "rs_apparch_config_03_io_outbound_dependency_direction_tests/mod.rs"]
mod rs_apparch_config_03_io_outbound_dependency_direction_tests;
