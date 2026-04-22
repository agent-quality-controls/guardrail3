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
        crate_input(G3RsApparchLayer::Types, "types/core/Cargo.toml"),
        crate_input(G3RsApparchLayer::Types, "types/shared/Cargo.toml"),
        crate_input(G3RsApparchLayer::Logic, "logic/service/Cargo.toml"),
        crate_input(G3RsApparchLayer::IoOutbound, "io/outbound/db/Cargo.toml"),
    ];
    G3RsApparchConfigChecksInput {
        crate_dependency_checks: vec![G3RsApparchCrateDependencyChecksInput {
            krate: crates
                .first()
                .expect("types test input should contain a source crate")
                .clone(),
            internal_dependencies: edges
                .iter()
                .filter(|(from, _)| *from == "types/core/Cargo.toml")
                .filter_map(|(_, to)| {
                    crates
                        .iter()
                        .find(|krate| krate.cargo_rel_path == *to)
                        .cloned()
                        .map(|target| G3RsApparchBoundDependency {
                            dep_name: target.crate_name.clone(),
                            kind: G3RsApparchDependencyKind::Dependency,
                            target,
                        })
                })
                .collect(),
        }],
        crate_purity_checks: Vec::new(),
        patch_bypass_checks: Vec::<G3RsApparchPatchBypassChecksInput>::new(),
        same_layer_cycles_check: G3RsApparchSameLayerCyclesChecksInput {
            edges: Vec::new(),
        },
    }
}

pub(super) fn run_rule(input: &G3RsApparchConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let crate_check = input
        .crate_dependency_checks
        .first()
        .expect("types test input should contain a source crate");

    crate::rs_apparch_config_01_types_dependency_direction::check(crate_check, &mut results);

    results
}
