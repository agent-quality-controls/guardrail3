use g3rs_apparch_types::{
    G3RsApparchConfigChecksInput, G3RsApparchCrate, G3RsApparchDependencyEdge,
    G3RsApparchDependencyKind, G3RsApparchLayer, G3RsApparchRustPolicyState,
};

pub(super) fn input() -> G3RsApparchConfigChecksInput {
    G3RsApparchConfigChecksInput {
        crates: vec![
            crate_input("types/core", G3RsApparchLayer::Types),
            crate_input("logic/service", G3RsApparchLayer::Logic),
            crate_input("io/outbound/db", G3RsApparchLayer::IoOutbound),
            crate_input("io/inbound/http", G3RsApparchLayer::IoInbound),
        ],
        dependency_edges: vec![
            edge("io/inbound/http", "types/core"),
            edge("io/inbound/http", "logic/service"),
            edge("io/inbound/http", "io/outbound/db"),
        ],
        external_dependencies: Vec::new(),
        patch_bypasses: Vec::new(),
        rust_policy: G3RsApparchRustPolicyState::Missing,
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

fn edge(from_rel_dir: &str, to_rel_dir: &str) -> G3RsApparchDependencyEdge {
    G3RsApparchDependencyEdge {
        from_cargo_rel_path: format!("{from_rel_dir}/Cargo.toml"),
        to_cargo_rel_path: format!("{to_rel_dir}/Cargo.toml"),
        dep_name: to_rel_dir
            .rsplit('/')
            .next()
            .expect("fixture dependency path should end with a crate name")
            .to_owned(),
        kind: G3RsApparchDependencyKind::Dependency,
    }
}
