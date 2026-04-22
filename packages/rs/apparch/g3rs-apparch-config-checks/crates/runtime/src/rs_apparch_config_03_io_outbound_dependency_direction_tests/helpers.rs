use g3rs_apparch_types::{
    G3RsApparchBoundDependency, G3RsApparchConfigChecksInput, G3RsApparchCrate,
    G3RsApparchCrateDependencyChecksInput, G3RsApparchDependencyKind, G3RsApparchLayer,
    G3RsApparchPatchBypassChecksInput, G3RsApparchSameLayerCyclesChecksInput,
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
    let crates = vec![
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
    ];

    G3RsApparchConfigChecksInput {
        crate_dependency_checks: crates
            .iter()
            .map(|krate| G3RsApparchCrateDependencyChecksInput {
                krate: krate.clone(),
                internal_dependencies: edges
                    .iter()
                    .filter(|(from, _)| *from == krate.cargo_rel_path)
                    .filter_map(|(_, to)| {
                        crates
                            .iter()
                            .find(|target| target.cargo_rel_path == *to)
                            .cloned()
                            .map(|target| G3RsApparchBoundDependency {
                                dep_name: target.crate_name.clone(),
                                kind: G3RsApparchDependencyKind::Dependency,
                                target,
                            })
                    })
                    .collect(),
            })
            .collect(),
        crate_purity_checks: Vec::new(),
        patch_bypass_checks: Vec::<G3RsApparchPatchBypassChecksInput>::new(),
        same_layer_cycles_check: G3RsApparchSameLayerCyclesChecksInput {
            edges: Vec::new(),
        },
    }
}

pub(super) fn run_rule(
    input: &G3RsApparchConfigChecksInput,
    source_cargo_rel_path: &str,
) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let crate_check = input
        .crate_dependency_checks
        .iter()
        .find(|check| check.krate.cargo_rel_path == source_cargo_rel_path)
        .expect("io/outbound test input should contain the requested source crate");

    crate::rs_apparch_config_03_io_outbound_dependency_direction::check(crate_check, &mut results);

    results
}
