use std::collections::BTreeMap;

use g3rs_apparch_types::{G3RsApparchCrate, G3RsApparchDependencyEdge};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-APPARCH-CONFIG-07";

pub(crate) fn check(
    krate: &G3RsApparchCrate,
    crates_by_path: &BTreeMap<String, &G3RsApparchCrate>,
    dependency_edges: &[G3RsApparchDependencyEdge],
    results: &mut Vec<G3CheckResult>,
) {
    let Some(source_layer) = krate.layer else {
        return;
    };
    let violating = dependency_edges
        .iter()
        .filter(|edge| edge.from_cargo_rel_path == krate.cargo_rel_path)
        .filter(|edge| edge.kind.is_dev())
        .filter_map(|edge| {
            crates_by_path
                .get(&edge.to_cargo_rel_path)
                .copied()
                .map(|target| (edge, target))
        })
        .filter(|(_, target)| {
            target.layer.is_some_and(|target_layer| {
                crate::run::forbidden_runtime_dependency(source_layer, target_layer)
            })
        })
        .collect::<Vec<_>>();

    if violating.is_empty() {
        return;
    }

    for (edge, target) in violating {
        let target_layer = target.layer.expect("filtered to layered target");
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "dev-dependency direction violation".to_owned(),
            format!(
                "{} crate `{}` dev-depends on forbidden {} crate `{}` via `{}`. Consider restructuring the test dependency instead of reaching across apparch layers.",
                crate::run::layer_label(source_layer),
                crate::run::display_crate(krate),
                crate::run::layer_label(target_layer),
                crate::run::display_crate(target),
                edge.kind.label()
            ),
            Some(krate.cargo_rel_path.clone()),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "rs_apparch_config_07_dev_dependency_direction_tests/mod.rs"]
mod rs_apparch_config_07_dev_dependency_direction_tests;
