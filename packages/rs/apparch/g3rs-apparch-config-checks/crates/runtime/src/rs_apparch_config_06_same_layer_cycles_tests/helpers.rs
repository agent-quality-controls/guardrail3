use g3rs_apparch_types::{
    G3RsApparchCrate, G3RsApparchLayer, G3RsApparchSameLayerCyclesChecksInput,
    G3RsApparchSameLayerDependencyEdge,
};
use guardrail3_check_types::G3CheckResult;

pub(super) fn krate(layer: G3RsApparchLayer, cargo_rel_path: &str) -> G3RsApparchCrate {
    let rel_dir = cargo_rel_path.trim_end_matches("/Cargo.toml");
    G3RsApparchCrate {
        crate_name: rel_dir
            .rsplit('/')
            .next()
            .expect("fixture crate path should end with a crate name")
            .to_owned(),
        cargo_rel_path: cargo_rel_path.to_owned(),
        rel_dir: rel_dir.to_owned(),
        layer: Some(layer),
    }
}

pub(super) fn same_layer_edge(
    from: &G3RsApparchCrate,
    to: &G3RsApparchCrate,
) -> G3RsApparchSameLayerDependencyEdge {
    G3RsApparchSameLayerDependencyEdge {
        from: from.clone(),
        to: to.clone(),
    }
}

pub(super) fn run_rule(edges: &[G3RsApparchSameLayerDependencyEdge]) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let input = G3RsApparchSameLayerCyclesChecksInput {
        edges: edges.to_vec(),
    };
    crate::rs_apparch_config_06_same_layer_cycles::check(&input, &mut results);
    results
}
