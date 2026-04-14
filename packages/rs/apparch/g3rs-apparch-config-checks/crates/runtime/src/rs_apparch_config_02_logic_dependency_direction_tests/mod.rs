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

fn input(edges: &[(&str, &str)]) -> G3RsApparchConfigChecksInput {
    G3RsApparchConfigChecksInput {
        crates: vec![
            crate_input(G3RsApparchLayer::Logic, "logic/service/Cargo.toml"),
            crate_input(G3RsApparchLayer::Logic, "logic/shared/Cargo.toml"),
            crate_input(G3RsApparchLayer::IoInbound, "io/inbound/http/Cargo.toml"),
            crate_input(G3RsApparchLayer::IoOutbound, "io/outbound/db/Cargo.toml"),
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

#[test]
fn forbidden_io_dependency_fires() {
    let results = crate::check(&input(&[("logic/service/Cargo.toml", "io/outbound/db/Cargo.toml")]));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-02")
        .expect("logic violation");

    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.file(), Some("logic/service/Cargo.toml"));
}

#[test]
fn forbidden_io_inbound_dependency_fires() {
    let results = crate::check(&input(&[("logic/service/Cargo.toml", "io/inbound/http/Cargo.toml")]));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-02")
        .expect("logic io/inbound violation");

    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.file(), Some("logic/service/Cargo.toml"));
}

#[test]
fn forbidden_same_layer_dependency_fires() {
    let results = crate::check(&input(&[("logic/service/Cargo.toml", "logic/shared/Cargo.toml")]));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-02")
        .expect("logic same-layer violation");

    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.file(), Some("logic/service/Cargo.toml"));
}

#[test]
fn logic_depends_on_types_stays_allowed() {
    let results = crate::check(&input(&[("logic/service/Cargo.toml", "types/core/Cargo.toml")]));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-02")
        .expect("logic inventory");

    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}

#[test]
fn clean_logic_crate_emits_inventory() {
    let results = crate::check(&input(&[]));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-02")
        .expect("logic inventory");

    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}
