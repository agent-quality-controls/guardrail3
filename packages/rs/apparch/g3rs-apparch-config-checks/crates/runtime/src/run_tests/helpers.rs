use g3rs_apparch_types::{
    G3RsApparchBoundDependency, G3RsApparchConfigChecksInput, G3RsApparchCrate,
    G3RsApparchCrateDependencyChecksInput, G3RsApparchCratePurityChecksInput,
    G3RsApparchDependencyKind, G3RsApparchLayer, G3RsApparchPatchBypassChecksInput,
    G3RsApparchRustPolicyState, G3RsApparchSameLayerCyclesChecksInput,
};

pub(super) fn input() -> G3RsApparchConfigChecksInput {
    let types = crate_input("types/core", G3RsApparchLayer::Types);
    let logic = crate_input("logic/service", G3RsApparchLayer::Logic);
    let outbound = crate_input("io/outbound/db", G3RsApparchLayer::IoOutbound);
    let inbound = crate_input("io/inbound/http", G3RsApparchLayer::IoInbound);

    G3RsApparchConfigChecksInput {
        crate_dependency_checks: vec![
            G3RsApparchCrateDependencyChecksInput {
                krate: types.clone(),
                internal_dependencies: Vec::new(),
            },
            G3RsApparchCrateDependencyChecksInput {
                krate: logic.clone(),
                internal_dependencies: Vec::new(),
            },
            G3RsApparchCrateDependencyChecksInput {
                krate: outbound.clone(),
                internal_dependencies: Vec::new(),
            },
            G3RsApparchCrateDependencyChecksInput {
                krate: inbound.clone(),
                internal_dependencies: vec![
                    bound_dependency(&types),
                    bound_dependency(&logic),
                    bound_dependency(&outbound),
                ],
            },
        ],
        crate_purity_checks: vec![
            G3RsApparchCratePurityChecksInput {
                krate: types,
                external_dependencies: Vec::new(),
                rust_policy: G3RsApparchRustPolicyState::Missing,
            },
            G3RsApparchCratePurityChecksInput {
                krate: logic,
                external_dependencies: Vec::new(),
                rust_policy: G3RsApparchRustPolicyState::Missing,
            },
            G3RsApparchCratePurityChecksInput {
                krate: outbound,
                external_dependencies: Vec::new(),
                rust_policy: G3RsApparchRustPolicyState::Missing,
            },
            G3RsApparchCratePurityChecksInput {
                krate: inbound,
                external_dependencies: Vec::new(),
                rust_policy: G3RsApparchRustPolicyState::Missing,
            },
        ],
        patch_bypass_checks: Vec::<G3RsApparchPatchBypassChecksInput>::new(),
        same_layer_cycles_check: G3RsApparchSameLayerCyclesChecksInput {
            crates: Vec::new(),
            edges: Vec::new(),
        },
    }
}

fn crate_input(rel_dir: &str, layer: G3RsApparchLayer) -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: rel_dir
            .rsplit('/')
            .next()
            .expect("fixture crate path should end with a crate name")
            .to_owned(),
        cargo_rel_path: format!("{rel_dir}/Cargo.toml"),
        rel_dir: rel_dir.to_owned(),
        layer: Some(layer),
    }
}

fn bound_dependency(target: &G3RsApparchCrate) -> G3RsApparchBoundDependency {
    G3RsApparchBoundDependency {
        dep_name: target.crate_name.clone(),
        kind: G3RsApparchDependencyKind::Dependency,
        target: target.clone(),
    }
}
