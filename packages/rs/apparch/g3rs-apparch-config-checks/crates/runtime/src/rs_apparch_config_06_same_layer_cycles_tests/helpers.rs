use g3rs_apparch_types::{
    G3RsApparchCrate, G3RsApparchDependencyEdge, G3RsApparchDependencyKind, G3RsApparchLayer,
    G3RsApparchSameLayerCyclesChecksInput, G3RsApparchSameLayerDependencyEdge,
};
use guardrail3_check_types::G3CheckResult;

pub(super) fn krate(layer: G3RsApparchLayer, cargo_rel_path: &str) -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: cargo_rel_path.replace('/', "-"),
        cargo_rel_path: cargo_rel_path.to_owned(),
        rel_dir: cargo_rel_path.trim_end_matches("/Cargo.toml").to_owned(),
        layer: Some(layer),
    }
}

pub(super) fn edge(
    from: &str,
    to: &str,
    kind: G3RsApparchDependencyKind,
) -> G3RsApparchDependencyEdge {
    G3RsApparchDependencyEdge {
        from_cargo_rel_path: from.to_owned(),
        to_cargo_rel_path: to.to_owned(),
        dep_name: to.to_owned(),
        kind,
    }
}

pub(super) fn run_rule(
    crates: &[G3RsApparchCrate],
    edges: &[G3RsApparchDependencyEdge],
) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let input = G3RsApparchSameLayerCyclesChecksInput {
        edges: edges
            .iter()
            .filter(|edge| !edge.kind.is_dev())
            .filter_map(|edge| {
                let from = crates
                    .iter()
                    .find(|krate| krate.cargo_rel_path == edge.from_cargo_rel_path)?;
                let to = crates
                    .iter()
                    .find(|krate| krate.cargo_rel_path == edge.to_cargo_rel_path)?;
                (from.layer == to.layer).then(|| G3RsApparchSameLayerDependencyEdge {
                    from: from.clone(),
                    to: to.clone(),
                })
            })
            .collect(),
    };
    crate::rs_apparch_config_06_same_layer_cycles::check(&input, &mut results);
    results
}
