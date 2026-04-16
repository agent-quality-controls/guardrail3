use std::collections::BTreeMap;

use g3rs_apparch_types::{
    G3RsApparchConfigChecksInput, G3RsApparchCrate, G3RsApparchDependencyEdge,
    G3RsApparchDependencyKind, G3RsApparchLayer, G3RsApparchRustPolicyState,
};
use guardrail3_check_types::G3CheckResult;

fn crate_input(layer: G3RsApparchLayer, cargo_rel_path: &str) -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: cargo_rel_path.replace('/', "-"),
        cargo_rel_path: cargo_rel_path.to_owned(),
        rel_dir: cargo_rel_path.trim_end_matches("/Cargo.toml").to_owned(),
        layer: Some(layer),
    }
}

pub(super) fn input(edges: &[(&str, &str)]) -> G3RsApparchConfigChecksInput {
    G3RsApparchConfigChecksInput {
        crates: vec![
            crate_input(G3RsApparchLayer::IoOutbound, "io/outbound/db/Cargo.toml"),
            crate_input(G3RsApparchLayer::IoOutbound, "io/outbound/cache/Cargo.toml"),
            crate_input(
                G3RsApparchLayer::IoOutbound,
                "io/outbound/report/crates/runtime/Cargo.toml",
            ),
            crate_input(
                G3RsApparchLayer::IoOutbound,
                "io/outbound/report/crates/assertions/Cargo.toml",
            ),
            crate_input(G3RsApparchLayer::Logic, "logic/service/Cargo.toml"),
            crate_input(G3RsApparchLayer::IoInbound, "io/inbound/http/Cargo.toml"),
            crate_input(G3RsApparchLayer::Types, "types/core/Cargo.toml"),
        ],
        dependency_edges: edges
            .iter()
            .map(|(from, to)| G3RsApparchDependencyEdge {
                from_cargo_rel_path: (*from).to_owned(),
                to_cargo_rel_path: (*to).to_owned(),
                dep_name: (*to).to_owned(),
                kind: G3RsApparchDependencyKind::Dependency,
            })
            .collect(),
        external_dependencies: Vec::new(),
        patch_bypasses: Vec::new(),
        rust_policy: G3RsApparchRustPolicyState::Missing,
    }
}

pub(super) fn run_rule(
    input: &G3RsApparchConfigChecksInput,
    source_cargo_rel_path: &str,
) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let crates_by_path = input
        .crates
        .iter()
        .map(|krate| (krate.cargo_rel_path.clone(), krate))
        .collect::<BTreeMap<_, _>>();
    let krate = input
        .crates
        .iter()
        .find(|krate| krate.cargo_rel_path == source_cargo_rel_path)
        .expect("io/outbound test input should contain the requested source crate");

    crate::rs_apparch_config_03_io_outbound_dependency_direction::check(
        krate,
        &crates_by_path,
        &input.dependency_edges,
        &mut results,
    );

    results
}
