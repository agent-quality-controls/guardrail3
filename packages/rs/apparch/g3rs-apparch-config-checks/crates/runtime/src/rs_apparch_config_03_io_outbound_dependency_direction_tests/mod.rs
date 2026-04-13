use g3rs_apparch_types::{
    G3RsApparchConfigChecksInput, G3RsApparchCrate, G3RsApparchDependencyEdge, G3RsApparchLayer,
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
            crate_input(G3RsApparchLayer::IoOutbound, "io/outbound/db/Cargo.toml"),
            crate_input(G3RsApparchLayer::IoOutbound, "io/outbound/cache/Cargo.toml"),
            crate_input(G3RsApparchLayer::Logic, "logic/service/Cargo.toml"),
            crate_input(G3RsApparchLayer::IoInbound, "io/inbound/http/Cargo.toml"),
            crate_input(G3RsApparchLayer::Types, "types/core/Cargo.toml"),
        ],
        dependency_edges: edges
            .iter()
            .map(|(from, to)| G3RsApparchDependencyEdge {
                from_cargo_rel_path: (*from).to_owned(),
                to_cargo_rel_path: (*to).to_owned(),
            })
            .collect(),
    }
}

#[test]
fn forbidden_logic_dependency_fires() {
    let results = crate::check(&input(&[("io/outbound/db/Cargo.toml", "logic/service/Cargo.toml")]));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-03")
        .expect("outbound violation");

    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.file(), Some("io/outbound/db/Cargo.toml"));
}

#[test]
fn forbidden_io_inbound_dependency_fires() {
    let results = crate::check(&input(&[("io/outbound/db/Cargo.toml", "io/inbound/http/Cargo.toml")]));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-03")
        .expect("outbound io/inbound violation");

    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.file(), Some("io/outbound/db/Cargo.toml"));
}

#[test]
fn forbidden_same_layer_dependency_fires() {
    let results = crate::check(&input(&[("io/outbound/db/Cargo.toml", "io/outbound/cache/Cargo.toml")]));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-03")
        .expect("outbound same-layer violation");

    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.file(), Some("io/outbound/db/Cargo.toml"));
}

#[test]
fn clean_outbound_crate_emits_inventory() {
    let results = crate::check(&input(&[("io/outbound/db/Cargo.toml", "types/core/Cargo.toml")]));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-03")
        .expect("outbound inventory");

    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}
