use g3rs_apparch_types::{
    G3RsApparchConfigChecksInput, G3RsApparchCrate, G3RsApparchDependencyEdge,
    G3RsApparchDependencyKind, G3RsApparchLayer, G3RsApparchRustPolicyState,
};
use guardrail3_check_types::G3Severity;

fn crate_input(layer: G3RsApparchLayer, cargo_rel_path: &str) -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: cargo_rel_path.replace('/', "-"),
        cargo_rel_path: cargo_rel_path.to_owned(),
        rel_dir: cargo_rel_path.trim_end_matches("/Cargo.toml").to_owned(),
        layer: Some(layer),
    }
}

fn input(edge: Option<G3RsApparchDependencyEdge>) -> G3RsApparchConfigChecksInput {
    G3RsApparchConfigChecksInput {
        crates: vec![
            crate_input(G3RsApparchLayer::Logic, "logic/service/Cargo.toml"),
            crate_input(G3RsApparchLayer::Types, "types/core/Cargo.toml"),
            crate_input(G3RsApparchLayer::IoOutbound, "io/outbound/db/Cargo.toml"),
        ],
        dependency_edges: edge.into_iter().collect(),
        external_dependencies: Vec::new(),
        patch_bypasses: Vec::new(),
        rust_policy: G3RsApparchRustPolicyState::Missing,
    }
}

#[test]
fn forbidden_dev_dependency_warns() {
    let results = crate::check(&input(Some(G3RsApparchDependencyEdge {
        from_cargo_rel_path: "logic/service/Cargo.toml".to_owned(),
        to_cargo_rel_path: "io/outbound/db/Cargo.toml".to_owned(),
        dep_name: "db-outbound".to_owned(),
        kind: G3RsApparchDependencyKind::TargetDevDependency,
    })));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-07")
        .expect("dev-direction warning");

    assert_eq!(result.severity(), G3Severity::Warn);
}

#[test]
fn runtime_dependency_is_not_reported_by_dev_rule() {
    let results = crate::check(&input(Some(G3RsApparchDependencyEdge {
        from_cargo_rel_path: "logic/service/Cargo.toml".to_owned(),
        to_cargo_rel_path: "io/outbound/db/Cargo.toml".to_owned(),
        dep_name: "db-outbound".to_owned(),
        kind: G3RsApparchDependencyKind::Dependency,
    })));

    assert!(results
        .iter()
        .all(|result| result.id() != "RS-APPARCH-CONFIG-07"));
}

#[test]
fn allowed_dev_dependency_stays_quiet() {
    let results = crate::check(&input(Some(G3RsApparchDependencyEdge {
        from_cargo_rel_path: "logic/service/Cargo.toml".to_owned(),
        to_cargo_rel_path: "types/core/Cargo.toml".to_owned(),
        dep_name: "types-core".to_owned(),
        kind: G3RsApparchDependencyKind::DevDependency,
    })));

    assert!(results
        .iter()
        .all(|result| result.id() != "RS-APPARCH-CONFIG-07"));
}
