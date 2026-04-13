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
            crate_input(G3RsApparchLayer::Types, "types/core/Cargo.toml"),
            crate_input(G3RsApparchLayer::Types, "types/shared/Cargo.toml"),
            crate_input(G3RsApparchLayer::Logic, "logic/service/Cargo.toml"),
            crate_input(G3RsApparchLayer::IoOutbound, "io/outbound/db/Cargo.toml"),
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
    let results = crate::check(&input(&[("types/core/Cargo.toml", "logic/service/Cargo.toml")]));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-01")
        .expect("types violation");

    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.file(), Some("types/core/Cargo.toml"));
    assert!(result.title().contains("depends on forbidden crate"));
}

#[test]
fn forbidden_same_layer_dependency_fires() {
    let results = crate::check(&input(&[("types/core/Cargo.toml", "types/shared/Cargo.toml")]));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-01")
        .expect("types same-layer violation");

    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.file(), Some("types/core/Cargo.toml"));
}

#[test]
fn clean_types_crate_emits_inventory() {
    let results = crate::check(&input(&[]));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-01")
        .expect("types inventory");

    assert_eq!(result.severity(), G3Severity::Info);
    assert_eq!(result.file(), Some("types/core/Cargo.toml"));
    assert!(result.inventory());
}
